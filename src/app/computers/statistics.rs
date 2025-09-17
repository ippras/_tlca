use crate::{app::panes::calculation::settings::Settings, utils::Hashed};
use egui::util::cache::{ComputerMut, FrameCache};
use itertools::Itertools;
use metadata::MetaDataFrame;
use polars::{
    error::PolarsResult,
    lazy::dsl::{max_horizontal, min_horizontal},
    prelude::*,
};
use std::{
    f64::consts::{E, FRAC_1_SQRT_2, SQRT_2},
    hash::{Hash, Hasher},
};
use tracing::instrument;

const LEFT: &str = "Left";
const RIGHT: &str = "Right";

/// Statistics computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Statistics computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.frame.value.clone().lazy();
        println!("Statistics 0: {}", lazy_frame.clone().collect().unwrap());
        let schema = lazy_frame.collect_schema()?;
        let names = schema
            .iter_names()
            .filter(|&name| name != "Composition" && name != "Species");
        let exprs = names
            .combinations_with_replacement(2)
            .map(|combination| -> PolarsResult<_> {
                let left = combination[0];
                let right = combination[1];
                let name = format!("{left}/{right}");
                let left = || {
                    col(left.clone())
                        .struct_()
                        .field_by_name("Mean")
                        .fill_null(0)
                };
                let right = || {
                    col(right.clone())
                        .struct_()
                        .field_by_name("Mean")
                        .fill_null(0)
                };
                Ok(as_struct(vec![
                    // Similarity between two data points
                    euclidean_distance(left(), right()).alias("EuclideanDistance"),
                    chebyshev_distance(left(), right()).alias("ChebyshevDistance"),
                    manhattan_distance(left(), right()).alias("ManhattanDistance"),
                    // Similarity between two sets
                    cosine_distance(left(), right()).alias("CosineDistance"),
                    jaccard_distance(left(), right())?.alias("JaccardDistance"),
                    // Similarity between two probability distributions
                    bhattacharyya_distance(left(), right()).alias("BhattacharyyaDistance"),
                    hellinger_distance(left(), right()).alias("HellingerDistance"),
                    jensen_shannon_distance(left(), right()).alias("JensenShannonDistance"),
                    // Correlation between two
                    pearson_corr(left(), right()).alias("PearsonCorrelation"),
                    spearman_rank_corr(left(), right(), false).alias("SpearmanRankCorrelation"),
                ])
                .alias(name))
            })
            .collect::<PolarsResult<Vec<_>>>()?;
        lazy_frame = lazy_frame.select(exprs);
        // lazy_frame = lazy_frame
        //     .select([
        //         nth(2)
        //             .as_expr()
        //             .struct_()
        //             .field_by_name("Mean")
        //             .fill_null(0)
        //             .alias(LEFT),
        //         nth(3)
        //             .as_expr()
        //             .struct_()
        //             .field_by_name("Mean")
        //             .fill_null(0)
        //             .alias(RIGHT),
        //     ])
        //     .select([
        //         // Similarity between two data points
        //         euclidean_distance(col(LEFT), col(RIGHT)).alias("EuclideanDistance"),
        //         chebyshev_distance(col(LEFT), col(RIGHT)).alias("ChebyshevDistance"),
        //         manhattan_distance(col(LEFT), col(RIGHT)).alias("ManhattanDistance"),
        //         // Similarity between two sets
        //         cosine_distance(col(LEFT), col(RIGHT)).alias("CosineDistance"),
        //         jaccard_distance(col(LEFT), col(RIGHT))?.alias("JaccardDistance"),
        //         // Similarity between two probability distributions
        //         bhattacharyya_distance(col(LEFT), col(RIGHT)).alias("BhattacharyyaDistance"),
        //         hellinger_distance(col(LEFT), col(RIGHT)).alias("HellingerDistance"),
        //         jensen_shannon_distance(col(LEFT), col(RIGHT)).alias("JensenShannonDistance"),
        //         // Correlation between two
        //         pearson_corr(col(LEFT), col(RIGHT)).alias("PearsonCorrelation"),
        //         spearman_rank_corr(col(LEFT), col(RIGHT), false).alias("SpearmanRankCorrelation"),
        //     ]);
        println!("Statistics 1: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Statistics key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a Hashed<DataFrame>,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.frame.hash(state);
        self.settings.kind.hash(state);
    }
}

/// Statistics value
type Value = DataFrame;

fn simpson_coefficient(a: Expr, b: Expr) -> PolarsResult<Expr> {
    Ok(min_horizontal([a.clone(), b.clone()])?.sum() / min_horizontal([a.sum(), b.sum()])?)
}

fn sørensen_coefficient(a: Expr, b: Expr) -> PolarsResult<Expr> {
    Ok(lit(1) - lit(2) * min_horizontal([a.clone(), b.clone()])?.sum() / (a.sum() + b.sum()))
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

fn jaccard_distance(a: Expr, b: Expr) -> PolarsResult<Expr> {
    Ok(lit(1) - min_horizontal([a.clone(), b.clone()])?.sum() / max_horizontal([a, b])?.sum())
}

fn jensen_shannon_distance(mut a: Expr, mut b: Expr) -> Expr {
    fn kullback_leibler_divergence(a: Expr, b: Expr) -> Expr {
        (a.clone() * (a / b).log(lit(E))).fill_nan(0).sum()
    }

    a = a.clone() / a.sum();
    b = b.clone() / b.sum();
    let m = (a.clone() + b.clone()) / lit(2);
    (lit(0.5) * kullback_leibler_divergence(a, m.clone())
        + lit(0.5) * kullback_leibler_divergence(b, m))
    .sqrt()
}
