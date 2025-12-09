use crate::{
    app::states::fatty_acids::Settings,
    r#const::{FILTER, MEAN, SAMPLE, STANDARD_DEVIATION},
    utils::HashedDataFrame,
};
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
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &Settings) -> Self {
        Self {
            frame,
            percent: settings.percent,
            precision: settings.precision,
            significant: settings.significant,
        }
    }
}

/// Format value
type Value = DataFrame;

fn format(key: Key) -> PolarsResult<LazyFrame> {
    let lazy_frame = key.frame.data_frame.clone().lazy();
    let mut exprs = vec![col(LABEL), col(FATTY_ACID).fatty_acid().format()];
    let mut sum = Vec::new();
    for name in key
        .frame
        .data_frame
        .get_column_names_str()
        .into_iter()
        .filter(|&name| !matches!(name, LABEL | FATTY_ACID | FILTER))
    {
        exprs.push(
            as_struct(vec![
                format_mean(col(name).clone().struct_().field_by_name(MEAN), key),
                format_standard_deviation(
                    col(name)
                        .clone()
                        .struct_()
                        .field_by_name(STANDARD_DEVIATION),
                    key,
                )?,
                format_array(col(name).struct_().field_by_name(SAMPLE), key)?,
            ])
            .alias(name),
        );
        sum.push(
            as_struct(vec![
                format_mean(col(name).clone().struct_().field_by_name(MEAN).sum(), key),
                format_standard_deviation(
                    col(name)
                        .clone()
                        .struct_()
                        .field_by_name(STANDARD_DEVIATION)
                        .pow(2)
                        .sum()
                        .sqrt(),
                    key,
                )?,
                // TODO: Следить когда добавят возможность складывать массивы поэлементно
                // format_array(
                //     col(name)
                //         .struct_()
                //         .field_by_name(ARRAY)
                //         .arr()
                //         .eval(element().sum(), false),
                //     key,
                // )?,
                format_array(
                    concat_arr(vec![
                        col(name)
                            .struct_()
                            .field_by_name(SAMPLE)
                            .arr()
                            .to_struct(None)
                            .struct_()
                            .field_by_name("*")
                            .sum(),
                    ])?
                    .alias(SAMPLE),
                    key,
                )?,
            ])
            .alias(name),
        );
    }
    exprs.push(col(FILTER));
    concat_lf_diagonal(
        [
            lazy_frame.clone().select(exprs),
            lazy_frame.clone().select(sum),
        ],
        UnionArgs::default(),
    )
}

fn format_mean(expr: Expr, key: Key) -> Expr {
    format_float(expr, key).alias(MEAN)
}

fn format_standard_deviation(expr: Expr, key: Key) -> PolarsResult<Expr> {
    Ok(format_str("±{}", [format_float(expr, key)])?.alias(STANDARD_DEVIATION))
}

fn format_array(expr: Expr, key: Key) -> PolarsResult<Expr> {
    Ok(ternary_expr(
        expr.clone().arr().len().neq(1),
        format_str(
            "[{}]",
            [expr
                .arr()
                .eval(format_float(element(), key), false)
                .arr()
                .join(lit(", "), false)],
        )?,
        lit(NULL),
    ))
}

fn format_float(expr: Expr, key: Key) -> Expr {
    expr.percent_if(key.percent)
        .precision(key.precision, key.significant)
        .cast(DataType::String)
}
