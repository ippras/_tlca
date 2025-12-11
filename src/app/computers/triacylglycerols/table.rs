use crate::{
    app::states::triacylglycerols::{
        composition::{
            Composition, ECN_MONO, ECN_STEREO, MASS_MONO, MASS_STEREO, SPECIES_MONO,
            SPECIES_POSITIONAL, SPECIES_STEREO, TYPE_MONO, TYPE_POSITIONAL, TYPE_STEREO,
            UNSATURATION_MONO, UNSATURATION_STEREO,
        },
        settings::Settings,
    },
    r#const::{COMPOSITION, EM_DASH, MEAN, SAMPLE, SPECIES, STANDARD_DEVIATION, THRESHOLD},
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::expr::{ExprExt as _, ExprIfExt as _};

/// Table computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Table computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        lazy_frame = format(lazy_frame, key)?;
        let data_frame = lazy_frame.collect()?;
        Ok(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Table key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) composition: Composition,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &Settings) -> Self {
        Self {
            frame,
            composition: settings.composition,
            percent: settings.percent,
            precision: settings.precision,
            significant: settings.significant,
        }
    }
}

/// Table value
type Value = DataFrame;

fn format(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let mut exprs = vec![label(key)?, species(key)?];
    // let mut sum = Vec::new();
    for name in key
        .frame
        .get_column_names_str()
        .into_iter()
        .filter(|&name| !matches!(name, COMPOSITION | SPECIES | THRESHOLD))
    {
        exprs.push(
            as_struct(vec![
                col(name)
                    .struct_()
                    .field_by_name(MEAN)
                    .percent_if(key.percent)
                    .precision(key.precision, key.significant),
                col(name)
                    .struct_()
                    .field_by_name(STANDARD_DEVIATION)
                    .percent_if(key.percent)
                    .precision(key.precision + 1, key.significant),
                col(name).struct_().field_by_name(SAMPLE).arr().eval(
                    element()
                        .percent_if(key.percent)
                        .precision(key.precision, key.significant),
                    false,
                ),
            ])
            .alias(name),
        );
    }
    Ok(lazy_frame.clone().with_columns(exprs))
}

fn label(key: Key) -> PolarsResult<Expr> {
    Ok(match key.composition {
        ECN_MONO | MASS_MONO | UNSATURATION_MONO => format_str("({})", [col(COMPOSITION)])?,
        SPECIES_MONO | TYPE_MONO => format_str(
            "[{}/3; {}/3; {}/3]",
            [
                col(COMPOSITION).triacylglycerol().stereospecific_number1(),
                col(COMPOSITION).triacylglycerol().stereospecific_number2(),
                col(COMPOSITION).triacylglycerol().stereospecific_number3(),
            ],
        )?,
        ECN_STEREO | MASS_STEREO | SPECIES_STEREO | TYPE_STEREO | UNSATURATION_STEREO => {
            format_str(
                "[{}; {}; {}]",
                [
                    col(COMPOSITION).triacylglycerol().stereospecific_number1(),
                    col(COMPOSITION).triacylglycerol().stereospecific_number2(),
                    col(COMPOSITION).triacylglycerol().stereospecific_number3(),
                ],
            )?
        }
        SPECIES_POSITIONAL | TYPE_POSITIONAL => format_str(
            "[{}/2; {}; {}/2]",
            [
                col(COMPOSITION).triacylglycerol().stereospecific_number1(),
                col(COMPOSITION).triacylglycerol().stereospecific_number2(),
                col(COMPOSITION).triacylglycerol().stereospecific_number3(),
            ],
        )?,
    }
    .alias(LABEL))
}

fn species(key: Key) -> PolarsResult<Expr> {
    Ok(col(SPECIES)
        .list()
        .eval(as_struct(vec![
            {
                let label = || element().struct_().field_by_name(LABEL);
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
                let triacylglycerol = element()
                    .struct_()
                    .field_by_name(TRIACYLGLYCEROL)
                    .triacylglycerol();
                format_str(
                    "[{}; {}; {}]",
                    [
                        triacylglycerol
                            .clone()
                            .stereospecific_number1()
                            .fatty_acid()
                            .format(),
                        triacylglycerol
                            .clone()
                            .stereospecific_number2()
                            .fatty_acid()
                            .format(),
                        triacylglycerol
                            .stereospecific_number3()
                            .fatty_acid()
                            .format(),
                    ],
                )?
                .alias(TRIACYLGLYCEROL)
            },
            format_str(
                "[{}]",
                [element()
                    .struct_()
                    .field_by_name("Values")
                    .list()
                    .eval(ternary_expr(
                        element().is_not_null(),
                        format_float(element(), key),
                        lit(EM_DASH),
                    ))
                    .list()
                    .join(lit(", "), false)],
            )?
            .alias("Values"),
        ]))
        .alias(SPECIES))
}

// fn format_sum(index: usize, key: Key) -> PolarsResult<[Expr; 2]> {
//     Ok([
//         format_mean(
//             nth(index as _)
//                 .as_expr()
//                 .struct_()
//                 .field_by_name(MEAN)
//                 .sum(),
//             key,
//         ),
//         format_standard_deviation(
//             nth(index as _)
//                 .as_expr()
//                 .struct_()
//                 .field_by_name(STANDARD_DEVIATION)
//                 .pow(2)
//                 .sum()
//                 .sqrt(),
//             key,
//         )?,
//     ])
// }

// fn format_mean(expr: Expr, key: Key) -> Expr {
//     format_float(expr, key).alias(MEAN)
// }

// fn format_standard_deviation(expr: Expr, key: Key) -> PolarsResult<Expr> {
//     Ok(format_str("±{}", [format_float(expr, key)])?.alias(STANDARD_DEVIATION))
// }

fn format_float(expr: Expr, key: Key) -> Expr {
    expr.percent_if(key.percent)
        .precision(key.precision, key.significant)
}
