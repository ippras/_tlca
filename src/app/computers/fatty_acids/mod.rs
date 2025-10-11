use crate::{
    app::{
        panes::fatty_acids::state::Settings,
        parameters::{
            Filter, Sort,
            composition::{
                Composition, ECN_MONO, ECN_STEREO, MASS_MONO, MASS_STEREO, SPECIES_MONO,
                SPECIES_POSITIONAL, SPECIES_STEREO, TYPE_MONO, TYPE_POSITIONAL, TYPE_STEREO,
                UNSATURATION_MONO, UNSATURATION_STEREO,
            },
        },
    },
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use lipid::prelude::*;
use polars::prelude::*;
use std::convert::identity;
use tracing::instrument;

const ROUND_MASS: u32 = 1;

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
    pub(crate) composition: Composition,
    pub(crate) filter: Filter,
    pub(crate) sort: Sort,
    pub(crate) threshold: OrderedFloat<f64>,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &Settings) -> Self {
        Self {
            frames,
            composition: settings.parameters.composition,
            filter: settings.parameters.filter,
            sort: settings.parameters.sort,
            threshold: settings.parameters.threshold.into(),
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
    lazy_frame = values(lazy_frame)?;
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
fn values(mut lazy_frame: LazyFrame) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    let exprs = schema
        .iter_names()
        .filter_map(|name| {
            if name != LABEL && name != FATTY_ACID {
                Some(name)
            } else {
                None
            }
        })
        .map(|name| {
            let mean = col(name.clone())
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS123)
                .struct_()
                .field_by_name("Mean");
            let standard_deviation = col(name.clone())
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS123)
                .struct_()
                .field_by_name("StandardDeviation");
            let array = col(name.clone())
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS123)
                .struct_()
                .field_by_name("Array");
            ternary_expr(
                mean.clone().neq(0),
                as_struct(vec![
                    mean.alias("Mean"),
                    standard_deviation.alias("StandardDeviation"),
                    array.alias("Array"),
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
    match key.filter {
        Filter::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (AND)
            lazy_frame = lazy_frame.filter(all_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_null()])?);
        }
    }
    // Threshold
    // Значение в одном или более столбцах больше threshold
    lazy_frame = lazy_frame.filter(any_horizontal([all()
        .exclude_cols([LABEL, FATTY_ACID])
        .as_expr()
        .struct_()
        .field_by_name("Mean")
        .gt(key.threshold.into_inner())])?);
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
                    all().exclude_cols([LABEL, FATTY_ACID]).as_expr(),
                ],
                SortMultipleOptions::new(),
            );
        }
        Sort::Value => {
            lazy_frame = lazy_frame.sort_by_exprs(
                [all().exclude_cols([LABEL, FATTY_ACID]).as_expr()],
                SortMultipleOptions::new()
                    .with_order_descending(true)
                    .with_nulls_last(true),
            );
        }
    }
    lazy_frame
}

pub(crate) mod format;
pub(crate) mod metrics;
