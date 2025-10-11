pub(crate) use self::{
    format::{Computed as DisplayComputed, Key as DisplayKey},
    moments::{Computed as MomentsComputed, Key as MomentsKey},
    triacylglycerols::{Computed as TriacylglycerolsComputed, Key as TriacylglycerolsKey},
};

pub(crate) mod fatty_acids;
pub(crate) mod metrics;

mod format;
mod moments;
mod triacylglycerols;
