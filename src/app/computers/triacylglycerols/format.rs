use crate::{
    app::states::triacylglycerols::{
        composition::{
            Composition, ECN_MONO, ECN_STEREO, MASS_MONO, MASS_STEREO, SPECIES_MONO,
            SPECIES_POSITIONAL, SPECIES_STEREO, TYPE_MONO, TYPE_POSITIONAL, TYPE_STEREO,
            UNSATURATION_MONO, UNSATURATION_STEREO,
        },
        settings::Settings,
    },
    r#const::{MEAN, STANDARD_DEVIATION},
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::expr::{ExprExt as _, ExprIfExt as _};

/// Display computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Display computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let lazy_frame = format(key)?;
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
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) column: usize,
    pub(crate) composition: Composition,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, column: usize, settings: &Settings) -> Self {
        Self {
            frame,
            column,
            composition: settings.composition,
            percent: settings.percent,
            precision: settings.precision,
            significant: false,
        }
    }
}

/// Display value
type Value = DataFrame;

fn format(key: Key) -> PolarsResult<LazyFrame> {
    let mut lazy_frame = key.frame.data_frame.clone().lazy();
    match key.column {
        1 => {
            lazy_frame = lazy_frame.select([format_label(key)?, format_species(key)?]);
        }
        index => {
            let values = lazy_frame.clone().select(format_value(index, key)?);
            let sum = lazy_frame.select(format_sum(index, key)?);
            lazy_frame = concat_lf_diagonal([values, sum], UnionArgs::default())?;
        }
    }
    Ok(lazy_frame)
}

fn format_label(key: Key) -> PolarsResult<Expr> {
    let expr = match key.composition {
        ECN_MONO | MASS_MONO | UNSATURATION_MONO => format_str("({})", [col("Composition")])?,
        SPECIES_MONO | TYPE_MONO => format_str(
            "[{}/3; {}/3; {}/3]",
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
                "[{}; {}; {}]",
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
            "[{}/2; {}; {}/2}",
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
    };
    Ok(expr.alias(LABEL))
}

fn format_species(key: Key) -> PolarsResult<Expr> {
    Ok(col("Species")
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
                let triacylglycerol = || element().struct_().field_by_name(TRIACYLGLYCEROL);
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
            format_str(
                "[{}]",
                [element()
                    .struct_()
                    .field_by_name("Values")
                    .list()
                    .eval(ternary_expr(
                        element().is_not_null(),
                        format_float(element(), key),
                        lit("-"),
                    ))
                    .list()
                    .join(lit(", "), false)],
            )?
            .alias("Values"),
        ]))
        .alias("Species"))
}

fn format_sum(index: usize, key: Key) -> PolarsResult<[Expr; 2]> {
    Ok([
        format_mean(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name(MEAN)
                .sum(),
            key,
        ),
        format_standard_deviation(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name(STANDARD_DEVIATION)
                .pow(2)
                .sum()
                .sqrt(),
            key,
        )?,
    ])
}

fn format_value(index: usize, key: Key) -> PolarsResult<[Expr; 2]> {
    Ok([
        format_mean(nth(index as _).as_expr().struct_().field_by_name(MEAN), key),
        format_standard_deviation(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name(STANDARD_DEVIATION),
            key,
        )?,
    ])
}

fn format_mean(expr: Expr, key: Key) -> Expr {
    format_float(expr, key).alias(MEAN)
}

fn format_standard_deviation(expr: Expr, key: Key) -> PolarsResult<Expr> {
    Ok(format_str("±{}", [format_float(expr, key)])?.alias(STANDARD_DEVIATION))
}

fn format_float(expr: Expr, key: Key) -> Expr {
    expr.percent_if(key.percent)
        .precision(key.precision, key.significant)
        .cast(DataType::String)
}
