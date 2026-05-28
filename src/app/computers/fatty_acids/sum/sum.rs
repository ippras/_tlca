use crate::{
    app::states::fatty_acids::settings::{Index, Indices, Join, Settings, StereospecificNumbers},
    r#const::{MAJOR, MEAN, SAMPLE, STANDARD_DEVIATION, VALUE},
    utils::{HashedDataFrame, polars::eval_arr},
};
use const_format::formatcp;
use egui::util::cache::{ComputerMut, FrameCache};
use fatty_acid_expressions::r#const::sum::{
    CFA, D9, D12, EPA_AND_DHA, LCFA, MCFA, MUFA, NUFA, O3, O6, O9, PUFA, SCFA, SFA, TFA, UFA, VLCFA,
};
use lipid::prelude::*;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::num::NonZeroI8;
use tracing::instrument;
use widgets::settings::{Array, array::Item};

pub(crate) const VALUE_: &str = formatcp!("^{VALUE}_.+$");

/// Sum expressions computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Sum expressions computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        lazy_frame = values(lazy_frame, key);
        lazy_frame = compute(lazy_frame, key)?;
        let data_frame = lazy_frame.collect()?;
        Ok(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Sum expressions key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) ddof: u8,
    pub(crate) expressions: &'a Array,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: OrderedFloat<f64>,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            ddof: settings.mean.ddof,
            expressions: &settings.expressions.sum,
            precision: settings.precision.precision,
            significant: settings.precision.significant,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: settings.threshold.auto,
        }
    }
}

/// Sum expressions value
type Value = DataFrame;

/// Value
fn values(lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    lazy_frame.with_columns([col(VALUE_)
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Compute
// fn compute(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
//     let mut lazy_frames = Vec::with_capacity(key.indices.len());
//     for index in key.indices.iter().filter(|index| index.visible) {
//         let mut exprs = vec![lit(Series::new(
//             PlSmallStr::from_static(INDEX),
//             [index.name.clone()],
//         ))];
//         for name in key
//             .frame
//             .schema()
//             .iter_names()
//             .filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID | THRESHOLD))
//         {
//             let array = eval_arr(col(name.clone()).struct_().field_by_name(SAMPLE), |expr| {
//                 compute_index(&index.name, expr)
//             })?;
//             exprs.push(
//                 as_struct(vec![
//                     array
//                         .clone()
//                         .arr()
//                         .mean()
//                         .precision(key.precision, key.significant)
//                         .alias(MEAN),
//                     array
//                         .clone()
//                         .arr()
//                         .std(key.ddof)
//                         .precision(key.precision + 1, key.significant)
//                         .alias(STANDARD_DEVIATION),
//                     array
//                         .arr()
//                         .eval(element().precision(key.precision, key.significant), false)
//                         .alias(SAMPLE),
//                 ])
//                 .alias(name.clone()),
//             );
//         }
//         lazy_frames.push(lazy_frame.clone().select(exprs))
//     }
//     concat(lazy_frames, Default::default())
// }
fn compute(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    // Names
    let mut exprs =
        vec![lit(Series::from_iter(key.expressions.iter().filter_map(
            |item| item.visible.then_some(item.name.as_str()),
        ))
        .with_name(PlSmallStr::from_static(INDEX)))];
    // Values
    for name in key
        .frame
        .schema()
        .iter_names()
        .filter(|name| name.starts_with(formatcp!("{VALUE}_")))
    {
        let expr = concat_arr(
            key.expressions
                .iter()
                .filter(|item| item.visible)
                .map(|item| eval_arr(col(name.clone()), |expr| compute_item(item, expr)))
                .collect::<PolarsResult<_>>()?,
        )?
        .explode(ExplodeOptions {
            empty_as_null: true,
            keep_nulls: true,
        })
        .alias(name.clone());
        exprs.push(expr);
    }
    Ok(lazy_frame.select(exprs))
}

fn compute_item(item: &Item, expr: Expr) -> Expr {
    // "Saturated" => col(FATTY_ACID).fatty_acid().sum_saturated(expr),
    // "Monounsaturated" => col(FATTY_ACID).fatty_acid().sum_monounsaturated(expr),
    // "Polyunsaturated" => col(FATTY_ACID).fatty_acid().sum_polyunsaturated(expr),
    // "Unsaturated" => col(FATTY_ACID).fatty_acid().sum_unsaturated(expr, None),
    // "Unsaturated-9" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .sum_unsaturated(expr, NonZeroI8::new(-9)),
    // "Unsaturated-6" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .sum_unsaturated(expr, NonZeroI8::new(-6)),
    // "Unsaturated-3" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .sum_unsaturated(expr, NonZeroI8::new(-3)),
    // "Unsaturated9" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .sum_unsaturated(expr, NonZeroI8::new(9)),
    // "Trans" => col(FATTY_ACID).fatty_acid().sum_trans(expr),
    // "EicosapentaenoicAndDocosahexaenoic" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .eicosapentaenoic_and_docosahexaenoic(expr),
    // "FishLipidQuality" => col(FATTY_ACID).fatty_acid().fish_lipid_quality(expr),
    // "HealthPromotingIndex" => col(FATTY_ACID).fatty_acid().health_promoting_index(expr),
    // "HypocholesterolemicToHypercholesterolemic" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .hypocholesterolemic_to_hypercholesterolemic(expr),
    // "IndexOfAtherogenicity" => col(FATTY_ACID).fatty_acid().index_of_atherogenicity(expr),
    // "IndexOfThrombogenicity" => col(FATTY_ACID).fatty_acid().index_of_thrombogenicity(expr),
    // "LinoleicToAlphaLinolenic" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .linoleic_to_alpha_linolenic(expr),
    // "Polyunsaturated-6ToPolyunsaturated-3" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .polyunsaturated_6_to_polyunsaturated_3(expr),
    // "PolyunsaturatedToSaturated" => col(FATTY_ACID)
    //     .fatty_acid()
    //     .polyunsaturated_to_saturated(expr),
    // "UnsaturationIndex" => col(FATTY_ACID).fatty_acid().unsaturation_index(expr),
    // name => {
    //     println!("name: {name}");
    //     lit(NULL)
    // }
    match &*item.name {
        EPA_AND_DHA => col(FATTY_ACID)
            .fatty_acid()
            .eicosapentaenoic_and_docosahexaenoic(expr),
        // By chain length
        SCFA => expr
            .filter(col(FATTY_ACID).fatty_acid().carbon().lt_eq(5))
            .sum(),
        MCFA => expr
            .filter(
                col(FATTY_ACID)
                    .fatty_acid()
                    .carbon()
                    .gt_eq(6)
                    .and(col(FATTY_ACID).fatty_acid().carbon().lt_eq(12)),
            )
            .sum(),
        LCFA => expr
            .filter(
                col(FATTY_ACID)
                    .fatty_acid()
                    .carbon()
                    .gt_eq(13)
                    .and(col(FATTY_ACID).fatty_acid().carbon().lt_eq(21)),
            )
            .sum(),
        VLCFA => expr
            .filter(col(FATTY_ACID).fatty_acid().carbon().gt_eq(22))
            .sum(),
        // By unsaturated bounds
        // By count
        // NUFA => col(FATTY_ACID).fatty_acid().sum_saturated(expr),
        MUFA => col(FATTY_ACID).fatty_acid().sum_monounsaturated(expr),
        PUFA => col(FATTY_ACID).fatty_acid().sum_polyunsaturated(expr),
        SFA => col(FATTY_ACID).fatty_acid().sum_saturated(expr),
        UFA => col(FATTY_ACID).fatty_acid().sum_unsaturated(expr, None),
        // By offset
        D9 => col(FATTY_ACID)
            .fatty_acid()
            .sum_unsaturated(expr, NonZeroI8::new(9)),
        D12 => col(FATTY_ACID)
            .fatty_acid()
            .sum_unsaturated(expr, NonZeroI8::new(12)),
        O9 => col(FATTY_ACID)
            .fatty_acid()
            .sum_unsaturated(expr, NonZeroI8::new(-9)),
        O6 => col(FATTY_ACID)
            .fatty_acid()
            .sum_unsaturated(expr, NonZeroI8::new(-6)),
        O3 => col(FATTY_ACID)
            .fatty_acid()
            .sum_unsaturated(expr, NonZeroI8::new(-3)),
        // By parity
        CFA => col(FATTY_ACID).fatty_acid().sum_conjugated(expr),
        // By pattern
        TFA => col(FATTY_ACID).fatty_acid().sum_trans(expr),
        "IodineValue" => (expr * col(FATTY_ACID).fatty_acid().iodine_value()).sum(),
        _ => unreachable!(),
    }
}
