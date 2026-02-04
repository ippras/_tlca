use crate::{
    app::states::{
        fatty_acids::settings::{Settings, StereospecificNumbers},
        settings::Filter,
    },
    r#const::{MEAN, SAMPLE, STANDARD_DEVIATION, THRESHOLD},
    utils::{HashedDataFrame, polars::eval_arr},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;

/// Table computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Table computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        lazy_frame = unnest(lazy_frame, key);
        lazy_frame = filter(lazy_frame, key)?;
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
    pub(crate) ddof: u8,
    pub(crate) filter: Filter,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &Settings) -> Self {
        Self {
            frame,
            // ddof: settings.ddof,
            ddof: 1,
            filter: settings.filter,
            percent: settings.percent,
            precision: settings.precision,
            significant: settings.significant,
            stereospecific_numbers: settings.stereospecific_numbers,
        }
    }
}

/// Table value
type Value = DataFrame;

/// Unnest
fn unnest(lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    lazy_frame.with_columns([all()
        .exclude_cols([LABEL, FATTY_ACID, THRESHOLD])
        .as_expr()
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Filter
fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let expr = all().exclude_cols([LABEL, FATTY_ACID, THRESHOLD]).as_expr();
    lazy_frame = lazy_frame.filter(match key.filter {
        Filter::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (AND)
            all_horizontal([expr.is_not_null()])?
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            any_horizontal([expr.is_not_null()])?
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            any_horizontal([expr.is_null()])?
        }
    });
    Ok(lazy_frame)
}

/// Format
fn format(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let mut exprs = vec![col(LABEL), col(FATTY_ACID).fatty_acid().format()];
    let mut sum = Vec::new();
    for name in key
        .frame
        .data_frame
        .get_column_names_str()
        .into_iter()
        .filter(|&name| !matches!(name, LABEL | FATTY_ACID | THRESHOLD))
    {
        exprs.push(
            as_struct(vec![
                format_mean(col(name).struct_().field_by_name(MEAN), key),
                format_standard_deviation(
                    col(name).struct_().field_by_name(STANDARD_DEVIATION),
                    key,
                ),
                format_sample(col(name).struct_().field_by_name(SAMPLE), key),
            ])
            .alias(name),
        );
        let array = eval_arr(col(name).struct_().field_by_name(SAMPLE), |expr| {
            expr.filter(THRESHOLD).sum()
        })?;
        sum.push(
            as_struct(vec![
                format_mean(array.clone().arr().mean().alias(MEAN), key),
                format_standard_deviation(
                    array.clone().arr().std(key.ddof).alias(STANDARD_DEVIATION),
                    key,
                ),
                format_sample(array.alias(SAMPLE), key),
            ])
            .alias(name),
        );
    }
    exprs.push(col(THRESHOLD));
    concat_lf_diagonal(
        [lazy_frame.clone().select(exprs), lazy_frame.select(sum)],
        UnionArgs::default(),
    )
}

fn format_mean(expr: Expr, key: Key) -> Expr {
    expr.percent(key.percent)
        .precision(key.precision, key.significant)
}

fn format_standard_deviation(expr: Expr, key: Key) -> Expr {
    expr.percent(key.percent)
        .precision(key.precision + 1, key.significant)
}

fn format_sample(expr: Expr, key: Key) -> Expr {
    expr.arr().eval(
        element()
            .percent(key.percent)
            .precision(key.precision, key.significant),
        false,
    )
}
