pub(crate) use self::{
    calculation::{Computed as CalculationComputed, Key as CalculationKey},
    display::{Computed as DisplayComputed, Key as DisplayKey, Kind as DisplayKind},
    metrics::{Computed as MetricsComputed, Key as MetricsKey},
};

mod calculation;
mod display;
mod metrics;
