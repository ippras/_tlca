use crate::{
    app::states::fatty_acids::settings::{Settings, StereospecificNumbers},
    r#const::{HIGHLIGHT, MAJOR, VALUE_},
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use tracing::instrument;
use widgets::settings::Major;

/// Select computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Select computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = select(lazy_frame, key);
        lazy_frame = major(lazy_frame, key)?;
        lazy_frame = highlight_sort_filter(lazy_frame, key);
        let data_frame = lazy_frame.collect()?;
        HashedDataFrame::new(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Select key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) major: &'a Major,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            major: &settings.major,
            stereospecific_numbers: settings.stereospecific_numbers,
        }
    }
}

/// Select value
type Value = HashedDataFrame;

/// Select
fn select(lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    lazy_frame.with_columns([col(VALUE_)
        .struct_()
        .field_by_name(key.stereospecific_numbers.id())
        .name()
        .keep()])
}

/// Major column
fn major(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let major = if key.major.is_auto {
        // Берем среднее значение массива, так как иначе пришлось бы сравнивать все повторности попарно
        // Значение в любом из столбцов больше или равно major
        any_horizontal([col(VALUE_)
            .arr()
            .mean()
            .fill_null(0)
            .gt_eq(key.major.auto.0)])?
    } else {
        lit(true)
        // lit(Series::from_iter(&key.major.manual))
    };
    lazy_frame = lazy_frame.with_column(major.alias(MAJOR));
    Ok(lazy_frame)
}

/// Major column
fn highlight_sort_filter(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    if key.major.highlight_sort_filter.filter {
        lazy_frame = lazy_frame.filter(col(MAJOR));
    } else if key.major.highlight_sort_filter.sort {
        lazy_frame = lazy_frame.sort_by_exprs(
            [col(MAJOR)],
            SortMultipleOptions::new()
                .with_maintain_order(true)
                .with_order_reversed(),
        );
    }
    let highlight = if key.major.highlight_sort_filter.highlight {
        col(MAJOR)
    } else {
        lit(true)
    };
    lazy_frame = lazy_frame.with_column(highlight.alias(HIGHLIGHT));
    lazy_frame
}
