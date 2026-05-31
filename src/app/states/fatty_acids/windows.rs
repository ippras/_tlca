use serde::{Deserialize, Serialize};

/// Windows
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Windows {
    pub open_expressions_ratio_biodiesel: bool,
    pub open_expressions_ratio_metabolic: bool,
    pub open_expressions_ratio_nutritional: bool,
    pub open_expressions_sum: bool,
    pub open_factors: bool,
    pub open_metrics: bool,
    pub open_settings: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            open_expressions_ratio_biodiesel: false,
            open_expressions_ratio_metabolic: false,
            open_expressions_ratio_nutritional: false,
            open_expressions_sum: false,
            open_factors: false,
            open_metrics: false,
            open_settings: false,
        }
    }
}
