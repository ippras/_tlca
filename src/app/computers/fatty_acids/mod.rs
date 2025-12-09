use crate::{
    app::states::{
        Filter, Sort,
        fatty_acids::{Settings, StereospecificNumbers},
    },
    r#const::{FILTER, MEAN, SAMPLE, STANDARD_DEVIATION},
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use tracing::instrument;

/// Fatty acids computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Fatty acids computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        let lazy_frame = compute(key)?;
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
    pub(crate) filter: Filter,
    pub(crate) sort: Sort,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: OrderedFloat<f64>,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &Settings) -> Self {
        Self {
            frames,
            filter: settings.filter,
            sort: settings.sort,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: settings.threshold,
        }
    }
}

/// Fatty acids value
type Value = HashedDataFrame;

fn compute(key: Key) -> PolarsResult<LazyFrame> {
    println!("Fatty acids frames: {:?}", key.frames);
    let mut lazy_frame = join(key)?;
    println!(
        "Fatty acids join: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = values(lazy_frame, key)?;
    println!(
        "Fatty acids values: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = filter(lazy_frame, key)?;
    println!(
        "Fatty acids filter: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = sort(lazy_frame, key);
    // println!(
    //     "Fatty acids sort: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    Ok(lazy_frame)
}

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
fn values(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    let exprs = schema
        .iter_names()
        .filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID))
        .map(|name| {
            let expr = col(name.as_str())
                .struct_()
                .field_by_name(key.stereospecific_numbers.id());
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
            .alias(name.clone())
        })
        .collect::<Vec<_>>();
    lazy_frame = lazy_frame.with_columns(exprs);
    Ok(lazy_frame)
}

/// Filter
fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let expr = all().exclude_cols([LABEL, FATTY_ACID]).as_expr();
    // Join
    let mut predicate = match key.filter {
        Filter::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (AND)
            all_horizontal([expr.clone().is_not_null()])?
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            any_horizontal([expr.clone().is_not_null()])?
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            any_horizontal([expr.clone().is_null()])?
        }
    };
    lazy_frame = lazy_frame.filter(predicate);
    // Threshold
    // Значение в одном или более столбцах больше threshold
    predicate = any_horizontal([expr
        .clone()
        .struct_()
        .field_by_name(MEAN)
        .gt_eq(key.threshold.0)
        .and(expr.is_not_null())])?;
    lazy_frame = lazy_frame.with_column(predicate.alias("Filter"));
    Ok(lazy_frame)
}

/// Sort
fn sort(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    match key.sort {
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
                [all().exclude_cols([LABEL, FATTY_ACID, FILTER]).as_expr()],
                SortMultipleOptions::new()
                    .with_maintain_order(true)
                    .with_order_descending(true)
                    .with_nulls_last(true),
            );
        }
    }
    lazy_frame
}

pub(crate) mod factors;
pub(crate) mod indices;
pub(crate) mod metrics;
pub(crate) mod table;
