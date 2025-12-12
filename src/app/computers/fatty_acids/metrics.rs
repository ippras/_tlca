use crate::{
    app::states::{
        fatty_acids::settings::{Settings, StereospecificNumbers},
        settings::{Filter, Metric},
    },
    r#const::{MEAN, THRESHOLD},
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::f64::consts::{E, FRAC_1_SQRT_2};
use tracing::instrument;

/// Metrics computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Metrics computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        lazy_frame = unnest(lazy_frame, key);
        lazy_frame = filter(lazy_frame, key)?;
        // println!("Metrics 0: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = compute(lazy_frame, key)?;
        // println!("Metrics 1: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Metrics key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) ddof: u8,
    pub(crate) filter: Filter,
    pub(crate) metric: Metric,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: OrderedFloat<f64>,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &Settings) -> Self {
        Self {
            frame,
            ddof: 1,
            filter: settings.filter,
            metric: settings.metric,
            precision: settings.precision,
            significant: settings.significant,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: settings.threshold.auto,
        }
    }
}

/// Metrics value
type Value = DataFrame;

/// Unnest
fn unnest(lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    lazy_frame.with_columns([all()
        .exclude_cols([LABEL, FATTY_ACID, THRESHOLD])
        .as_expr()
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Filter
fn filter(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let expr = all().exclude_cols([LABEL, FATTY_ACID, THRESHOLD]).as_expr();
    Ok(lazy_frame.filter(match key.filter {
        Filter::Intersection => all_horizontal([expr.is_not_null()])?,
        Filter::Union => any_horizontal([expr.is_not_null()])?,
        Filter::Difference => any_horizontal([expr.is_null()])?,
    }))
}

/// Compute
fn compute(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let names = key.frame.schema().iter_names();
    let mut exprs = Vec::with_capacity(names.len());
    for name in names.filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID | THRESHOLD)) {
        // Метрики сравниваем по среднему, потому как сравнивать повторности
        // пришлось бы попарно все пары.
        let left = col(name.as_str())
            .struct_()
            .field_by_name(MEAN)
            .fill_null(0);
        let right = all()
            .exclude_cols([LABEL, FATTY_ACID, THRESHOLD])
            .as_expr()
            .struct_()
            .field_by_name(MEAN)
            .fill_null(0);
        let metric = match key.metric {
            // Similarity between two discrete probability distributions
            Metric::HellingerDistance => hellinger_distance(left, right),
            Metric::JensenShannonDistance => jensen_shannon_distance(left, right),
            Metric::BhattacharyyaDistance => bhattacharyya_distance(left, right),
            // Distance between two points
            Metric::ChebyshevDistance => chebyshev_distance(left, right),
            Metric::EuclideanDistance => euclidean_distance(left, right),
            Metric::ManhattanDistance => manhattan_distance(left, right),
            // Distance between two series
            Metric::CosineDistance => cosine_distance(left, right),
            Metric::JaccardDistance => jaccard_distance(left, right),
            Metric::OverlapDistance => overlap_distance(left, right),
        };
        exprs.push(
            concat_arr(vec![metric.precision(key.precision, key.significant)])?.alias(name.clone()),
        );
    }
    lazy_frame = lazy_frame.select(exprs).explode(all());
    Ok(lazy_frame)
}

// fn hierarchical_cluster(data_frame: DataFrame) {
// use linfa::{Dataset, DatasetBase, dataset::Records, traits::Transformer as _};
// use linfa_hierarchical::HierarchicalCluster;
// use linfa_kernel::{Kernel, KernelMethod};
//
//     // let dataset = linfa_datasets::iris();
//     let array = data_frame
//         .to_ndarray::<Float64Type>(IndexOrder::default())
//         .unwrap();
//     let dataset = Dataset::new(array, Default::default());
//     // Dataset::new(data, targets)
//     //     .map_targets(|x| *x as usize)
//     //     .with_feature_names(feature_names);
//     // let t = DatasetBase::from(data_frame.to_ndarray(IndexOrder::default()).unwrap());
//     let kernel = Kernel::params()
//         .method(KernelMethod::Gaussian(1.0))
//         .transform(array);
//     // let kernel = HierarchicalCluster::default()
//     //     .num_clusters(3)
//     //     .transform(kernel)
//     //     .unwrap();
//     // for (id, target) in kernel
//     //     .targets()
//     //     .iter()
//     //     .zip(dataset.into().targets().into_iter())
//     // {
//     //     let name = match *target {
//     //         0 => "setosa",
//     //         1 => "versicolor",
//     //         2 => "virginica",
//     //         _ => unreachable!(),
//     //     };
//     //     print!("({id} {name}) ");
//     // }
// }

fn overlap_distance(a: Expr, b: Expr) -> Expr {
    lit(1) - min(a.clone(), b.clone()).sum() / min(a.sum(), b.sum())
}

fn sørensen_coefficient(a: Expr, b: Expr) -> Expr {
    lit(1) - lit(2) * min(a.clone(), b.clone()).sum() / (a.sum() + b.sum())
}

fn bhattacharyya_distance(a: Expr, b: Expr) -> Expr {
    -(a * b).sqrt().sum().log(lit(E))
}

fn hellinger_distance(a: Expr, b: Expr) -> Expr {
    lit(FRAC_1_SQRT_2) * (a.sqrt() - b.sqrt()).pow(2).sum().sqrt()
}

fn euclidean_distance(a: Expr, b: Expr) -> Expr {
    (a - b).pow(2).sum().sqrt()
}

fn chebyshev_distance(a: Expr, b: Expr) -> Expr {
    (a - b).abs().max()
}

fn manhattan_distance(a: Expr, b: Expr) -> Expr {
    (a - b).abs().sum()
}

fn cosine_distance(a: Expr, b: Expr) -> Expr {
    lit(1) - (a.clone() * b.clone()).sum() / (a.pow(2).sum().sqrt() * b.pow(2).sum().sqrt())
}

fn bray_curtis_dissimilarity(a: Expr, b: Expr) -> Expr {
    (a.clone() - b.clone()).abs().sum() / (a + b).sum()
}

fn jaccard_distance(a: Expr, b: Expr) -> Expr {
    // Ok(lit(1) - min_horizontal([a.clone(), b.clone()])?.sum() / max_horizontal([a, b])?.sum())
    lit(1) - min(a.clone(), b.clone()).sum() / max(a, b).sum()
}

fn jensen_shannon_distance(mut a: Expr, mut b: Expr) -> Expr {
    pub const SQRT_LN_2: f64 = 0.83255461115769768820626950400765053927898406982421875_f64;

    fn kullback_leibler_divergence(a: Expr, b: Expr) -> Expr {
        (a.clone() * (a / b).log(lit(E))).fill_nan(0).sum()
    }

    a = a.clone() / a.sum();
    b = b.clone() / b.sum();
    let m = (a.clone() + b.clone()) / lit(2);
    (lit(0.5) * kullback_leibler_divergence(a, m.clone())
        + lit(0.5) * kullback_leibler_divergence(b, m))
    .sqrt()
        / lit(SQRT_LN_2)
}

fn max(a: Expr, b: Expr) -> Expr {
    ternary_expr(a.clone().gt_eq(b.clone()), a, b)
}

fn min(a: Expr, b: Expr) -> Expr {
    ternary_expr(a.clone().lt_eq(b.clone()), a, b)
}
