use crate::{
    app::states::fatty_acids::settings::Settings,
    r#const::{HIGHLIGHT, NAME, VALUE},
    utils::HashedDataFrame,
};
use const_format::formatcp;
use egui::util::cache::{ComputerMut, FrameCache};
use fatty_acid_expressions::r#const::sum::{
    CFA, D9, D12, EPA_AND_DHA, LCFA, MCFA, MUFA, NUFA, O3, O6, O9, PUFA, SCFA, SFA, TFA, UFA, VLCFA,
};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::num::NonZeroI8;
use tracing::instrument;
use widgets::settings::{Array as SumArray, HighlightSortFilter, Precision, array::Item};

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
        lazy_frame = compute(lazy_frame, key)?;
        println!("compute: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = highlight_sort_filter(lazy_frame, key)?;
        println!("threshold: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = format(lazy_frame, key)?;
        println!("format: {}", lazy_frame.clone().collect().unwrap());
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
    pub(crate) expressions: &'a SumArray,
    pub(crate) precision: Precision,
    pub(crate) highlight_sort_filter: HighlightSortFilter,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            ddof: settings.msd.ddof,
            expressions: &settings.expressions.sum,
            precision: settings.precision,
            highlight_sort_filter: settings.highlight_sort_filter,
        }
    }
}

/// Sum expressions value
type Value = DataFrame;

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
    let visible = key.expressions.iter().filter(|item| item.visible);
    // Names
    let mut exprs = vec![lit(Series::from_iter(
        visible.clone().map(|item| item.name.as_str()),
    )
    .with_name(PlSmallStr::from_static(NAME)))];
    // Values
    for name in key
        .frame
        .schema()
        .iter_names()
        .filter(|name| name.starts_with(formatcp!("{VALUE}_")))
    {
        let expr = concat_list(
            visible
                .clone()
                .map(|item| eval_arr(col(name.clone()), |expr| Ok(compute_item(item, expr))))
                .collect::<PolarsResult<Vec<_>>>()?,
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

fn highlight_sort_filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let predicate = any_horizontal([col(VALUE_).arr().agg(element().gt(0.0).any(true))])?;
    if key.highlight_sort_filter.filter {
        lazy_frame = lazy_frame.filter(predicate.clone());
    } else if key.highlight_sort_filter.sort {
        lazy_frame = lazy_frame.sort_by_exprs(
            [predicate.clone()],
            SortMultipleOptions::new()
                .with_maintain_order(true)
                .with_order_descending(true),
        );
    }
    lazy_frame = lazy_frame.with_column(
        if key.highlight_sort_filter.highlight {
            predicate
        } else {
            lit(true)
        }
        .alias(HIGHLIGHT),
    );
    Ok(lazy_frame)
}

fn format(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    Ok(lazy_frame.with_columns(
        schema
            .iter_names()
            .filter(|name| name.starts_with(formatcp!("{VALUE}_")))
            .map(|name| {
                Array::builder()
                    .expr(col(name.clone()))
                    .ddof(key.ddof)
                    .percent(key.precision.percent)
                    .precision(key.precision.precision)
                    .significant(key.precision.significant)
                    .build()
            })
            .collect::<Vec<_>>(),
    ))
}
