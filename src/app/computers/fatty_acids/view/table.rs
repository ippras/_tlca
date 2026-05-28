use crate::{
    app::states::fatty_acids::settings::{Join, Settings, StereospecificNumbers},
    r#const::{MAJOR, MEAN, SAMPLE, STANDARD_DEVIATION, VALUE, VALUE_},
    utils::{HashedDataFrame, polars::eval_arr},
};
use const_format::formatcp;
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use widgets::settings::Threshold;

/// Table computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Table computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = value(lazy_frame, key);
        println!("unnest: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = filter(lazy_frame, key)?;
        println!("filter_by_none: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = format(lazy_frame, key)?;
        println!(
            "format: {}",
            lazy_frame
                .clone()
                .unnest(cols(["Value_VIR-2233.2025-10-29"]), None)
                .collect()
                .unwrap()
        );
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
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: &'a Threshold,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            ddof: settings.mean.ddof,
            percent: settings.precision.percent,
            precision: settings.precision.precision,
            significant: settings.precision.significant,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: &settings.threshold,
        }
    }
}

/// Table value
type Value = DataFrame;

/// Value
fn value(lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    lazy_frame.with_columns([col(VALUE_)
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Filter
fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    // Majors for stereospecific numbers
    lazy_frame = lazy_frame.with_column(
        col(MAJOR)
            .struct_()
            .field_by_name(key.stereospecific_numbers.id())
            .name()
            .keep(),
    );
    // Filter minors
    if key.threshold.filter {
        lazy_frame = lazy_frame.filter(col(MAJOR));
    }
    Ok(lazy_frame)
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
