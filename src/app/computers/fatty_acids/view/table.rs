use crate::{
    app::states::fatty_acids::settings::{Filter, Settings, StereospecificNumbers},
    r#const::{FILTER, MEAN, SAMPLE, STANDARD_DEVIATION, VALUE, VALUE_},
    utils::{HashedDataFrame, polars::eval_arr},
};
use const_format::formatcp;
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
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = unnest(lazy_frame, key);
        println!("unnest: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = filter_by_none(lazy_frame, key)?;
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
            ddof: settings.ddof(),
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
    lazy_frame.with_columns([col(VALUE_)
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Filter
fn filter_by_none(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let filter_by_null = match key.filter {
        Filter::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (AND)
            all_horizontal([col(VALUE_).is_not_null()])?
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            any_horizontal([col(VALUE_).is_not_null()])?
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            any_horizontal([col(VALUE_).is_null()])?
        }
    };
    let filter_by_value = col(FILTER)
        .struct_()
        .field_by_name(key.stereospecific_numbers.id());
    Ok(lazy_frame.filter(filter_by_null.and(filter_by_value)))
}

/// Format
fn format(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    lazy_frame = lazy_frame.with_column(col(FATTY_ACID).fatty_acid().display());
    let schema = lazy_frame.collect_schema()?;
    lazy_frame = lazy_frame.with_columns(
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
                // .name().map(|name| name"^VALUE_", value, literal)
                // .name()
                // .map(PlanCallback::new(|name: PlSmallStr| {
                //     Ok(name.trim_prefix(formatcp!("{VALUE}_")).into())
                // }))
            })
            .collect::<Vec<_>>(),
    );
    Ok(lazy_frame)
}
