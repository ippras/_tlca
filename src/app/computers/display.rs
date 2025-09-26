use crate::{
    app::parameters::composition::{
        Composition, ECN_MONO, ECN_STEREO, MASS_MONO, MASS_STEREO, SPECIES_MONO,
        SPECIES_POSITIONAL, SPECIES_STEREO, TYPE_MONO, TYPE_POSITIONAL, TYPE_STEREO,
        UNSATURATION_MONO, UNSATURATION_STEREO,
    },
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::expr::ExprIfExt as _;
use std::hash::{Hash, Hasher};

/// Display computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Display computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.hashed_data_frame.data_frame.clone().lazy();
        match key.kind {
            // Kind::LabelAndTriacylglycerol => {
            //     lazy_frame = lazy_frame.select([label()?, triacylglycerol()?]);
            // }
            Kind::Composition { composition } => {
                lazy_frame = lazy_frame.select([label(composition)?, species(key.percent)?]);
            }
            Kind::Value { index } => {
                lazy_frame = lazy_frame.select([value(index, key.percent)]);
            }
        }
        let data_frame = lazy_frame.collect()?;
        Ok(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Display key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) hashed_data_frame: &'a HashedDataFrame,
    pub(crate) kind: Kind,
    pub(crate) percent: bool,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hashed_data_frame.hash.hash(state);
        self.kind.hash(state);
        self.percent.hash(state);
    }
}

/// Display value
type Value = DataFrame;

/// Display kind
#[derive(Clone, Copy, Debug, Hash)]
pub enum Kind {
    // LabelAndSpecies,
    Composition { composition: Composition },
    Value { index: usize },
}

fn label(composition: Composition) -> PolarsResult<Expr> {
    let expr = match composition {
        ECN_MONO | MASS_MONO | UNSATURATION_MONO => format_str("({})", [col("Composition")])?,
        SPECIES_MONO | TYPE_MONO => format_str(
            "({}, {}, {})", // { 1, 2, 3: {}, {}, {}}
            [
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number1(),
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number2(),
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number3(),
            ],
        )?,
        ECN_STEREO | MASS_STEREO | SPECIES_STEREO | TYPE_STEREO | UNSATURATION_STEREO => {
            format_str(
                "[{}; {}; {}]", // { 1: {}; 2: {}; 3: {}}
                [
                    col("Composition")
                        .triacylglycerol()
                        .stereospecific_number1(),
                    col("Composition")
                        .triacylglycerol()
                        .stereospecific_number2(),
                    col("Composition")
                        .triacylglycerol()
                        .stereospecific_number3(),
                ],
            )?
        }
        SPECIES_POSITIONAL | TYPE_POSITIONAL => format_str(
            "{1, 3: {}, {}; 2: {}}",
            [
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number1(),
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number3(),
                col("Composition")
                    .triacylglycerol()
                    .stereospecific_number2(),
            ],
        )?,
    };
    Ok(expr.alias(LABEL))
}

// fn label() -> PolarsResult<Expr> {
//     Ok(format_str(
//         "[{}; {}; {}]",
//         [
//             col(LABEL).triacylglycerol().stereospecific_number1(),
//             col(LABEL).triacylglycerol().stereospecific_number2(),
//             col(LABEL).triacylglycerol().stereospecific_number3(),
//         ],
//     )?
//     .alias(LABEL))
// }

fn species(percent: bool) -> PolarsResult<Expr> {
    Ok(col("Species")
        .list()
        .eval(as_struct(vec![
            {
                let label = || col("").struct_().field_by_name(LABEL);
                format_str(
                    "[{}; {}; {}]",
                    [
                        label().triacylglycerol().stereospecific_number1(),
                        label().triacylglycerol().stereospecific_number2(),
                        label().triacylglycerol().stereospecific_number3(),
                    ],
                )?
                .alias(LABEL)
            },
            {
                let triacylglycerol = || col("").struct_().field_by_name(TRIACYLGLYCEROL);
                format_str(
                    "[{}; {}; {}]",
                    [
                        triacylglycerol()
                            .triacylglycerol()
                            .stereospecific_number1()
                            .fatty_acid()
                            .format(),
                        triacylglycerol()
                            .triacylglycerol()
                            .stereospecific_number2()
                            .fatty_acid()
                            .format(),
                        triacylglycerol()
                            .triacylglycerol()
                            .stereospecific_number3()
                            .fatty_acid()
                            .format(),
                    ],
                )?
                .alias(TRIACYLGLYCEROL)
            },
            col("")
                .struct_()
                .field_by_name("Values")
                .list()
                .eval(col("").percent_if(percent)),
        ]))
        .alias("Species"))
}

// fn triacylglycerol() -> PolarsResult<Expr> {
//     Ok(format_str(
//         "[{}; {}; {}]",
//         [
//             col(TRIACYLGLYCEROL)
//                 .triacylglycerol()
//                 .stereospecific_number1()
//                 .fatty_acid()
//                 .format(),
//             col(TRIACYLGLYCEROL)
//                 .triacylglycerol()
//                 .stereospecific_number2()
//                 .fatty_acid()
//                 .format(),
//             col(TRIACYLGLYCEROL)
//                 .triacylglycerol()
//                 .stereospecific_number3()
//                 .fatty_acid()
//                 .format(),
//         ],
//     )?
//     .alias(TRIACYLGLYCEROL))
// }

fn value(index: usize, percent: bool) -> Expr {
    nth(index as _)
        .as_expr()
        .struct_()
        .field_by_name("*")
        .percent_if(percent)
}
