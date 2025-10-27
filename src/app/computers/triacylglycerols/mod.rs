use crate::{
    app::states::{
        Filter, Sort,
        triacylglycerols::{
            Settings,
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

/// Triacylglycerols computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Triacylglycerols computer
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

/// Triacylglycerols key
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

/// Triacylglycerols value
type Value = HashedDataFrame;

fn compute(key: Key) -> PolarsResult<LazyFrame> {
    println!("Triacylglycerols frames: {:?}", key.frames);
    let mut lazy_frame = join(key)?;
    // println!(
    //     "Triacylglycerols join: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    lazy_frame = compose(lazy_frame, key)?;
    // println!(
    //     "Triacylglycerols compose: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    lazy_frame = values(lazy_frame)?;
    // println!(
    //     "Triacylglycerols values: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    lazy_frame = filter(lazy_frame, key)?;
    // println!(
    //     "Triacylglycerols filter: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    lazy_frame = sort(lazy_frame, key);
    // println!(
    //     "Triacylglycerols sort: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    Ok(lazy_frame)
}

/// Join
fn join(key: Key) -> PolarsResult<LazyFrame> {
    let compute = |frame: &HashedMetaDataFrame| -> PolarsResult<LazyFrame> {
        Ok(frame.data.data_frame.clone().lazy().select([
            col(LABEL),
            col(TRIACYLGLYCEROL),
            col("Value").alias(frame.meta.format(".").to_string()),
        ]))
    };
    let mut lazy_frame = compute(&key.frames[0])?;
    for frame in &key.frames[1..] {
        lazy_frame = lazy_frame.join(
            compute(frame)?,
            [col(LABEL), col(TRIACYLGLYCEROL)],
            [col(LABEL), col(TRIACYLGLYCEROL)],
            JoinArgs {
                coalesce: JoinCoalesce::CoalesceColumns,
                maintain_order: MaintainOrderJoin::LeftRight,
                ..JoinArgs::new(JoinType::Full)
            },
        );
    }
    Ok(lazy_frame)
}

/// Compose
fn compose(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let by = [match key.composition {
        MASS_MONO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .mass(None)
            .round(ROUND_MASS, RoundMode::HalfToEven),
        MASS_STEREO => col(TRIACYLGLYCEROL).triacylglycerol().map_expr(|expr| {
            expr.fatty_acid()
                .mass(None)
                .round(ROUND_MASS, RoundMode::HalfToEven)
        }),
        ECN_MONO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .equivalent_carbon_number(),
        ECN_STEREO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .map_expr(|expr| expr.fatty_acid().equivalent_carbon_number()),
        SPECIES_MONO => col(LABEL).triacylglycerol().non_stereospecific(identity),
        SPECIES_POSITIONAL => col(LABEL).triacylglycerol().positional(identity),
        SPECIES_STEREO => col(LABEL),
        TYPE_MONO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .non_stereospecific(|expr| expr.fatty_acid().is_saturated().not())
            .triacylglycerol()
            .map_expr(|expr| expr.fatty_acid().r#type()),
        TYPE_POSITIONAL => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .positional(|expr| expr.fatty_acid().is_saturated().not())
            .triacylglycerol()
            .map_expr(|expr| expr.fatty_acid().r#type()),
        TYPE_STEREO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .map_expr(|expr| expr.fatty_acid().r#type()),
        UNSATURATION_MONO => col(TRIACYLGLYCEROL).triacylglycerol().unsaturation(),
        UNSATURATION_STEREO => col(TRIACYLGLYCEROL)
            .triacylglycerol()
            .map_expr(|expr| expr.fatty_acid().unsaturation()),
    }
    .alias("Composition")];
    let mut aggs = vec![
        as_struct(vec![
            col(LABEL),
            col(TRIACYLGLYCEROL),
            concat_list([all()
                .exclude_cols([LABEL, TRIACYLGLYCEROL])
                .as_expr()
                .struct_()
                .field_by_name("Mean")])?
            .alias("Values"),
        ])
        .alias("Species"),
    ];
    for frame in key.frames {
        let name = frame.meta.format(".").to_string();
        aggs.push(
            as_struct(vec![
                col(&name).struct_().field_by_name("Mean"),
                col(&name).struct_().field_by_name("StandardDeviation"),
            ])
            .alias(name),
        );
    }
    lazy_frame = lazy_frame.group_by(by).agg(aggs);
    Ok(lazy_frame)
}

/// Values
fn values(mut lazy_frame: LazyFrame) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    let exprs = schema
        .iter_names()
        .filter_map(|name| {
            if name != "Composition" && name != "Species" {
                Some(name)
            } else {
                None
            }
        })
        .map(|name| {
            let mean = || {
                col(name.clone())
                    .list()
                    .eval(col("").struct_().field_by_name("Mean"))
                    .list()
                    .sum()
            };
            let standard_deviation = || {
                col(name.clone())
                    .list()
                    .eval(col("").struct_().field_by_name("StandardDeviation").pow(2))
                    .list()
                    .sum()
                    .sqrt()
            };
            ternary_expr(
                mean().neq(0),
                as_struct(vec![
                    mean().alias("Mean"),
                    standard_deviation().alias("StandardDeviation"),
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
                .exclude_cols(["Composition", "Species"])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols(["Composition", "Species"])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols(["Composition", "Species"])
                .as_expr()
                .is_null()])?);
        }
    }
    // Threshold
    // Значение в одном или более столбцах больше threshold
    lazy_frame = lazy_frame.filter(any_horizontal([all()
        .exclude_cols(["Composition", "Species"])
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
                    col("Composition"),
                    all().exclude_cols(["Composition", "Species"]).as_expr(),
                ],
                SortMultipleOptions::new(),
            );
        }
        Sort::Value => {
            lazy_frame = lazy_frame.sort_by_exprs(
                [all().exclude_cols(["Composition", "Species"]).as_expr()],
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
pub(crate) mod moments;
