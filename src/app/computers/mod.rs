pub(crate) use self::{
    calculation::{Computed as CalculationComputed, Key as CalculationKey},
    display::{Computed as DisplayComputed, Key as DisplayKey, Kind as DisplayKind},
    statistics::{Computed as StatisticsComputed, Key as StatisticsKey},
};

mod calculation;
mod display;
mod statistics;
