use crate::{
    app::{
        computers::matches_schema,
        states::fatty_acids::settings::{Join, Settings},
    },
    r#const::{VALUE, VALUE_},
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::util::cache::{ComputerMut, FrameCache};
use itertools::Itertools;
use lipid::prelude::*;
use metadata::Metadata;
use polars::prelude::*;
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::LazyLock,
};
use tracing::instrument;
use widgets::settings::Major;

/// Input schema
pub(crate) const INPUT_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    Arc::new(Schema::from_iter([
        Field::new(PlSmallStr::from_static(LABEL), DataType::String),
        field!(FATTY_ACID),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS123),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS13),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
    ]))
});

/// Join computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Join computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        for frame in key.frames {
            matches_schema(&frame.data.data_frame, &INPUT_SCHEMA)?;
        }
        let mut lazy_frame = join(key)?;
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = filter(lazy_frame, key)?;
        let data_frame = lazy_frame.collect()?;
        HashedDataFrame::new(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Join key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [HashedMetaDataFrame],
    pub(crate) join: Join,
    pub(crate) major: &'a Major,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &'a Settings) -> Self {
        Self {
            frames,
            join: settings.join,
            major: &settings.major,
        }
    }
}

/// Join value
type Value = HashedDataFrame;

/// Join
fn join(key: Key) -> PolarsResult<LazyFrame> {
    let names = names(key);
    let compute = |frame: &HashedMetaDataFrame| -> PolarsResult<LazyFrame> {
        Ok(frame.data.data_frame.clone().lazy().select([
            col(LABEL),
            col(FATTY_ACID),
            as_struct(vec![
                col(STEREOSPECIFIC_NUMBERS123),
                col(STEREOSPECIFIC_NUMBERS13),
                col(STEREOSPECIFIC_NUMBERS2),
            ])
            .alias(format!("{VALUE}_{}", &names[&frame.meta])),
        ]))
    };
    let mut lazy_frame = compute(&key.frames[0])?;
    for frame in &key.frames[1..] {
        lazy_frame = lazy_frame.join(
            compute(frame)?,
            [col(LABEL), col(FATTY_ACID)],
            [col(LABEL), col(FATTY_ACID)],
            JoinArgs {
                coalesce: JoinCoalesce::CoalesceColumns,
                maintain_order: MaintainOrderJoin::LeftRight,
                ..JoinArgs::new(JoinType::Full)
            },
        );
    }
    Ok(lazy_frame)
}

fn names(key: Key<'_>) -> HashMap<&Metadata, String> {
    let mut names = HashMap::new();
    for frame in key.frames {
        match names.entry(frame.meta.display().build().to_string()) {
            Entry::Occupied(occupied) => {
                let meta: &Metadata = occupied.remove();
                names.insert(meta.display().date(true).build().to_string(), meta);
                names.insert(
                    frame.meta.display().date(true).build().to_string(),
                    &frame.meta,
                );
            }
            Entry::Vacant(vacant) => {
                vacant.insert(&frame.meta);
            }
        }
    }
    names.into_iter().map(|(key, value)| (value, key)).collect()
}

/// Filter
fn filter(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    Ok(lazy_frame.filter(match key.join {
        Join::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (Intersection)
            all_horizontal([col(VALUE_).is_not_null()])?
        }
        Join::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (Union)
            any_horizontal([col(VALUE_).is_not_null()])?
        }
        Join::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (Difference)
            any_horizontal([col(VALUE_).is_null()])?
        }
    }))
}
