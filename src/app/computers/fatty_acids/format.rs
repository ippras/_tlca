use crate::{app::states::fatty_acids::Settings, utils::HashedDataFrame};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::expr::{ExprExt as _, ExprIfExt as _};

/// Format computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Format computer
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

/// Format key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) column: usize,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, column: usize, settings: &Settings) -> Self {
        Self {
            frame,
            column,
            percent: settings.percent,
            precision: settings.precision,
            significant: false,
        }
    }
}

/// Format value
type Value = DataFrame;

fn format(key: Key) -> PolarsResult<LazyFrame> {
    let mut lazy_frame = key.frame.data_frame.clone().lazy();
    match key.column {
        1 => {
            lazy_frame = lazy_frame.select([col(LABEL), col(FATTY_ACID).fatty_acid().format()]);
        }
        index => {
            let values = lazy_frame.clone().select(format_value(index, key)?);
            let sum = lazy_frame.select(format_sum(index, key)?);
            lazy_frame = concat_lf_diagonal([values, sum], UnionArgs::default())?;
        }
    }
    Ok(lazy_frame)
}

fn format_sum(index: usize, key: Key) -> PolarsResult<[Expr; 2]> {
    Ok([
        format_mean(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name("Mean")
                .sum(),
            key,
        ),
        format_standard_deviation(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name("StandardDeviation")
                .pow(2)
                .sum()
                .sqrt(),
            key,
        )?,
    ])
}

fn format_value(index: usize, key: Key) -> PolarsResult<[Expr; 2]> {
    Ok([
        format_mean(
            nth(index as _).as_expr().struct_().field_by_name("Mean"),
            key,
        ),
        format_standard_deviation(
            nth(index as _)
                .as_expr()
                .struct_()
                .field_by_name("StandardDeviation"),
            key,
        )?,
    ])
}

fn format_mean(expr: Expr, key: Key) -> Expr {
    format_float(expr, key).alias("Mean")
}

fn format_standard_deviation(expr: Expr, key: Key) -> PolarsResult<Expr> {
    Ok(format_str("±{}", [format_float(expr, key)])?.alias("StandardDeviation"))
}

fn format_float(expr: Expr, key: Key) -> Expr {
    expr.percent_if(key.percent)
        .precision(key.precision, key.significant)
        .cast(DataType::String)
}
