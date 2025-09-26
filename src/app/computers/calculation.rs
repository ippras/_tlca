use crate::{
    app::parameters::{
        Filter, Parameters, Sort,
        composition::{
            ECN_MONO, ECN_STEREO, MASS_MONO, MASS_STEREO, SPECIES_MONO, SPECIES_POSITIONAL,
            SPECIES_STEREO, TYPE_MONO, TYPE_POSITIONAL, TYPE_STEREO, UNSATURATION_MONO,
            UNSATURATION_STEREO,
        },
    },
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use std::{
    convert::identity,
    hash::{Hash, Hasher},
};
use tracing::instrument;

const ROUND_MASS: u32 = 1;

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        // Join
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
        // println!("Calculation 0: {}", lazy_frame.clone().collect().unwrap());
        // Compose
        let expr = match key.parameters.composition {
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
        .alias("Composition");
        lazy_frame = lazy_frame.group_by([expr]).agg([
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
            all().exclude_cols([LABEL, TRIACYLGLYCEROL]).as_expr(),
        ]);
        // println!("Calculation 1: {}", lazy_frame.clone().collect().unwrap());
        let schema = lazy_frame.collect_schema()?;
        let exprs = schema
            .iter_fields()
            .filter_map(|field| {
                let name = field.name();
                if name != "Composition" && name != "Species" {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .map(|name| {
                let mean = || {
                    col(name.as_str())
                        .list()
                        .eval(col("").struct_().field_by_name("Mean"))
                        .list()
                        .sum()
                };
                ternary_expr(
                    mean().neq(0),
                    as_struct(vec![
                        mean().alias("Mean"), // col(name.as_str()).alias("Repetitions"),
                    ]),
                    lit(NULL),
                )
                .alias(name)
            })
            .collect::<Vec<_>>();
        lazy_frame = lazy_frame.with_columns(exprs);
        // println!("Calculation 2: {}", lazy_frame.clone().collect().unwrap());
        // Filter
        match key.parameters.filter {
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
        // println!("Calculation 3: {}", lazy_frame.clone().collect().unwrap());
        // Threshold
        // Значение в одном или более столбцах больше threshold
        lazy_frame = lazy_frame.filter(any_horizontal([all()
            .exclude_cols(["Composition", "Species"])
            .as_expr()
            .struct_()
            .field_by_name("Mean")
            .gt(key.parameters.threshold)])?);
        // println!("Calculation 4: {}", lazy_frame.clone().collect().unwrap());
        // Sort
        match key.parameters.sort {
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
        // println!("Calculation 5: {}", lazy_frame.clone().collect().unwrap());
        // let mean = |expr: Expr| -> PolarsResult<_> {
        //     Ok(as_struct(vec![
        //         (all()
        //             .exclude_cols([LABEL, TRIACYLGLYCEROL])
        //             .as_expr()
        //             .struct_()
        //             .field_by_name("Mean")
        //             - nth(2).as_expr().struct_().field_by_name("Mean"))
        //         .abs(),
        //     ])
        //     .alias(expr.meta().output_name()?))
        // };
        // if key.settings.kind == Kind::Difference {
        //     lazy_frame = lazy_frame.select([
        //         col(LABEL),
        //         col(TRIACYLGLYCEROL),
        //         mean(all().exclude_cols([LABEL, TRIACYLGLYCEROL]).as_expr())?,
        //     ]);
        //     // as_struct(vec![
        //     //     (all()
        //     //         .exclude_cols([LABEL, TRIACYLGLYCEROL])
        //     //         .as_expr()
        //     //         .struct_()
        //     //         .field_by_name("Mean")
        //     //         - nth(2).as_expr().struct_().field_by_name("Mean"))
        //     //     .abs(),
        //     // ]),
        // }
        // println!("lazy_frame: {:?}", lazy_frame.clone().collect()?);
        let mut data_frame = lazy_frame.collect()?;
        let hash = data_frame.hash_rows(None)?.xor_reduce().unwrap_or_default();
        Ok(HashedDataFrame { data_frame, hash })
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [HashedMetaDataFrame],
    pub(crate) parameters: &'a Parameters,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.frames.hash(state);
        self.parameters.hash(state);
    }
}

/// Calculation value
type Value = HashedDataFrame;
