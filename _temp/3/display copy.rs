use crate::utils::Hashed;
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::hash::{Hash, Hasher};

/// Display computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Display computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.data_frame.value.clone().lazy();
        match key.kind {
            Kind::LabelAndTriacylglycerol => {
                lazy_frame = lazy_frame.select([label()?, triacylglycerol()?]);
            }
            Kind::Value { index, percent } => {
                lazy_frame = lazy_frame.select([value(index, percent)]);
            }
        }
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
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) data_frame: &'a Hashed<DataFrame>,
    pub(crate) kind: Kind,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_frame.hash.hash(state);
        self.kind.hash(state);
    }
}

/// Display value
type Value = DataFrame;

/// Display kind
#[derive(Clone, Copy, Debug, Hash)]
pub enum Kind {
    LabelAndTriacylglycerol,
    Value { index: usize, percent: bool },
}

fn label() -> PolarsResult<Expr> {
    Ok(format_str(
        "[{}; {}; {}]",
        [
            col(LABEL).triacylglycerol().stereospecific_number1(),
            col(LABEL).triacylglycerol().stereospecific_number2(),
            col(LABEL).triacylglycerol().stereospecific_number3(),
        ],
    )?
    .alias(LABEL))
}

fn triacylglycerol() -> PolarsResult<Expr> {
    Ok(format_str(
        "[{}; {}; {}]",
        [
            col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .stereospecific_number1()
                .fatty_acid()
                .format(),
            col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .stereospecific_number2()
                .fatty_acid()
                .format(),
            col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .stereospecific_number3()
                .fatty_acid()
                .format(),
        ],
    )?
    .alias(TRIACYLGLYCEROL))
}

fn value(index: usize, percent: bool) -> Expr {
    nth(index as _)
        .as_expr()
        .struct_()
        .field_by_name("*")
        .percent_if(percent)
}
