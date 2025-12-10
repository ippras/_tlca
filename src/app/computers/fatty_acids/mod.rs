use crate::{
    app::states::{
        fatty_acids::settings::{Settings, StereospecificNumbers},
        settings::{Sort, Threshold},
    },
    r#const::{MEAN, SAMPLE, STANDARD_DEVIATION, THRESHOLD},
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use tracing::instrument;

/// Fatty acids computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Fatty acids computer
#[derive(Default)]
pub(crate) struct Computer;

// From:
// ┌───────┬───────────┬──────────────────────────┬─────────────────────────┬────────────────────────┐
// │ Label ┆ FattyAcid ┆ StereospecificNumbers123 ┆ StereospecificNumbers13 ┆ StereospecificNumbers2 │
// ┆ ---   ┆ ---       ┆ ---                      ┆ ---                     ┆ ---                    │
// │ str   ┆ struct[2] ┆ struct[3]                ┆ struct[3]               ┆ struct[3]              │
// ╞═══════╪═══════════╪══════════════════════════╪═════════════════════════╪════════════════════════╡
impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        let mut lazy_frame = join(key)?;
        lazy_frame = values(lazy_frame)?;
        lazy_frame = threshold(lazy_frame, key)?;
        lazy_frame = sort(lazy_frame, key);
        let data_frame = lazy_frame.collect()?;
        HashedDataFrame::new(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Fatty acids key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [HashedMetaDataFrame],
    pub(crate) sort: Option<Sort>,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: &'a Threshold,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &'a Settings) -> Self {
        Self {
            frames,
            sort: settings.sort,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: &settings.threshold,
        }
    }
}

/// Fatty acids value
type Value = HashedDataFrame;

/// Join
fn join(key: Key) -> PolarsResult<LazyFrame> {
    let compute = |frame: &HashedMetaDataFrame| -> PolarsResult<LazyFrame> {
        Ok(frame.data.data_frame.clone().lazy().select([
            col(LABEL),
            col(FATTY_ACID),
            as_struct(vec![
                col(STEREOSPECIFIC_NUMBERS123),
                col(STEREOSPECIFIC_NUMBERS13),
                col(STEREOSPECIFIC_NUMBERS2),
            ])
            .alias(frame.meta.format(".").to_string()),
        ]))
    };
    let mut lazy_frame = compute(&key.frames[0])?;
    for frame in &key.frames[1..] {
        lazy_frame = lazy_frame.join(
            compute(frame)?,
            [col(LABEL), col(FATTY_ACID)],
            [col(LABEL), col(FATTY_ACID)],
            JoinArgs {
                coalesce: JoinCoalesce::CoalesceColumns,
                maintain_order: MaintainOrderJoin::LeftRight,
                ..JoinArgs::new(JoinType::Full)
            },
        );
    }
    Ok(lazy_frame)
}

/// Values
fn values(mut lazy_frame: LazyFrame) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    let exprs = schema
        .iter_names()
        .filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID))
        .map(|name| {
            let field = |stereospecific_numbers: &str| {
                let expr = col(name.as_str())
                    .struct_()
                    .field_by_name(stereospecific_numbers);
                let mean = expr.clone().struct_().field_by_name(MEAN);
                let standard_deviation = expr.clone().struct_().field_by_name(STANDARD_DEVIATION);
                let sample = expr.struct_().field_by_name("Array");
                ternary_expr(
                    mean.clone().neq(0),
                    as_struct(vec![
                        mean.alias(MEAN),
                        standard_deviation.alias(STANDARD_DEVIATION),
                        sample.alias(SAMPLE),
                    ]),
                    lit(NULL),
                )
                .alias(stereospecific_numbers)
            };
            as_struct(vec![
                field(STEREOSPECIFIC_NUMBERS123),
                field(STEREOSPECIFIC_NUMBERS13),
                field(STEREOSPECIFIC_NUMBERS2),
            ])
            .alias(name.clone())
        })
        .collect::<Vec<_>>();
    lazy_frame = lazy_frame.with_columns(exprs);
    Ok(lazy_frame)
}

/// Threshold
fn threshold(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    // Значение в одном или более столбцах больше threshold,
    let predicate = any_horizontal([all()
        .exclude_cols([LABEL, FATTY_ACID])
        .as_expr()
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .struct_()
        .field_by_name(MEAN)
        .fill_null(0)
        .gt_eq(key.threshold.auto.0)])?;
    lazy_frame = lazy_frame.with_column(predicate.alias(THRESHOLD));
    if key.threshold.filter {
        lazy_frame = lazy_frame.filter(col(THRESHOLD));
    }
    if key.threshold.sort {
        lazy_frame = lazy_frame.sort(
            [THRESHOLD],
            SortMultipleOptions::new()
                .with_maintain_order(true)
                .with_order_descending(true),
        );
    }
    Ok(lazy_frame)
}

/// Sort
fn sort(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    if let Some(sort) = key.sort {
        match sort {
            Sort::Key => {
                lazy_frame = lazy_frame.sort_by_exprs(
                    [
                        col(FATTY_ACID).fatty_acid().carbon(),
                        col(FATTY_ACID).fatty_acid().double_bounds_unsaturation(),
                        col(FATTY_ACID).fatty_acid().indices(),
                        col(LABEL),
                    ],
                    SortMultipleOptions::new().with_maintain_order(true),
                );
            }
            Sort::Value => {
                lazy_frame = lazy_frame.sort_by_exprs(
                    [all().exclude_cols([LABEL, FATTY_ACID]).as_expr()],
                    SortMultipleOptions::new()
                        .with_maintain_order(true)
                        .with_order_descending(true)
                        .with_nulls_last(true),
                );
            }
        }
    }
    lazy_frame
}

// /// Filter
// fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
//     let expr = all().exclude_cols([LABEL, FATTY_ACID]).as_expr();
//     // Join
//     let mut predicate = match key.filter {
//         Filter::Intersection => {
//             // Значения отличные от нуля присутствуют во всех столбцах (AND)
//             all_horizontal([expr.clone().is_not_null()])?
//         }
//         Filter::Union => {
//             // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
//             any_horizontal([expr.clone().is_not_null()])?
//         }
//         Filter::Difference => {
//             // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
//             any_horizontal([expr.clone().is_null()])?
//         }
//     };
//     lazy_frame = lazy_frame.filter(predicate);
//     // Threshold
//     // Значение в одном или более столбцах больше threshold
//     predicate = any_horizontal([expr
//         .clone()
//         .struct_()
//         .field_by_name(MEAN)
//         .gt_eq(key.threshold.0)
//         .and(expr.is_not_null())])?;
//     lazy_frame = lazy_frame.with_column(predicate.alias("Filter"));
//     Ok(lazy_frame)
// }

pub(crate) mod factors;
pub(crate) mod indices;
pub(crate) mod metrics;
pub(crate) mod table;
