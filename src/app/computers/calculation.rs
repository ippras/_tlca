use crate::{
    app::panes::calculation::settings::{Kind, Settings},
    utils::Hashed,
};
use egui::util::cache::{ComputerMut, FrameCache};
use metadata::MetaDataFrame;
use polars::prelude::*;
use std::hash::{Hash, Hasher};
use tracing::instrument;

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        if key.frames.is_empty() {
            return Ok(DataFrame::empty());
        }
        let compute = |frame: &MetaDataFrame| -> PolarsResult<LazyFrame> {
            Ok(frame.data.clone().lazy().select([
                col("Triacylglycerol"),
                col("Value").alias(frame.meta.format(".").to_string()),
            ]))
        };
        let mut lazy_frame = compute(&key.frames[0])?;
        for frame in &key.frames[1..] {
            lazy_frame = lazy_frame.join(
                compute(frame)?,
                [col("Triacylglycerol")],
                [col("Triacylglycerol")],
                JoinArgs {
                    coalesce: JoinCoalesce::CoalesceColumns,
                    maintain_order: MaintainOrderJoin::LeftRight,
                    ..JoinArgs::new(JoinType::Full)
                },
            );
        }
        if key.settings.kind == Kind::Difference {
            lazy_frame = lazy_frame.select([
                col("Triacylglycerol"),
                (dtype_col(&DataType::Float64) - nth(1)).abs(),
            ]);
        }
        println!("lazy_frame: {:?}", lazy_frame.clone().collect()?);
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [Hashed<MetaDataFrame>],
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.frames.hash(state);
        self.settings.kind.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
