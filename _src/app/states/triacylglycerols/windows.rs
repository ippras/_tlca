use serde::{Deserialize, Serialize};

/// Windows
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Windows {
    pub open_metadata: bool,
    pub open_metrics: bool,
    pub open_moments: bool,
    pub open_settings: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            open_metadata: false,
            open_metrics: false,
            open_moments: false,
            open_settings: false,
        }
    }
}
