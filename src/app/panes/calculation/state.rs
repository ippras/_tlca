use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) struct State {
    pub(crate) add_table_row: bool,
    pub(crate) delete_table_row: Option<usize>,
    pub(crate) open_settings_window: bool,
    pub(crate) reset_table_state: bool,
    pub(crate) take_firts_table_rows: Option<usize>,
    pub(crate) take_last_table_rows: Option<usize>,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            add_table_row: false,
            delete_table_row: None,
            open_settings_window: false,
            reset_table_state: false,
            take_firts_table_rows: None,
            take_last_table_rows: None,
        }
    }
}
