use crate::{
    app::states::fatty_acids::settings::{Settings, Sort},
    r#const::{MAJOR, VALUE, VALUE_},
    utils::HashedDataFrame,
};
use const_format::formatcp;
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::r#const::LABEL;
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
        lazy_frame = sort(lazy_frame, key);
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
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
    pub(crate) sort: (Option<Sort>, bool),
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            ddof: settings.mean.ddof,
            percent: settings.precision.percent,
            precision: settings.precision.precision,
            significant: settings.precision.significant,
            sort: (settings.sort, settings.major.sort),
        }
    }
}

/// Table value
type Value = DataFrame;

/// Sort
fn sort(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    if let Some(sort) = key.sort.0 {
        let sort_options = SortMultipleOptions::default().with_maintain_order(true);
        lazy_frame = match (sort, key.sort.1) {
            (Sort::Key, false) => lazy_frame.sort_by_exprs([col(LABEL)], sort_options),
            (Sort::Key, true) => lazy_frame.sort_by_exprs(
                [col(MAJOR), col(LABEL)],
                sort_options.with_order_descending_multi([true, false]),
            ),
            (Sort::Value, false) => lazy_frame.sort_by_exprs([col(VALUE_)], sort_options),
            (Sort::Value, true) => lazy_frame.sort_by_exprs(
                [col(MAJOR), col(VALUE_)],
                sort_options.with_order_descending_multi([true, false]),
            ),
        };
    }
    lazy_frame
}

/// Format
fn format(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    Ok(lazy_frame.with_columns(
        schema
            .iter_names()
            .filter(|name| name.starts_with(formatcp!("{VALUE}_")))
            .map(|name| {
                Array::builder()
                    .expr(col(name.clone()))
                    .ddof(key.ddof)
                    .percent(key.percent)
                    .precision(key.precision)
                    .significant(key.significant)
                    .build()
            })
            .collect::<Vec<_>>(),
    ))
}
