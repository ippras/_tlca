use super::ID_SOURCE;
use egui::{Context, Id};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// static SETTINGS: LazyLock<Id> = LazyLock::new(|| Id::new(ID_SOURCE).with("Settings"));
// static WINDOWS: LazyLock<Id> = LazyLock::new(|| Id::new(ID_SOURCE).with("Windows"));
const SETTINGS: &str = "Settings";
const WINDOWS: &str = "Windows";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) struct State {
    pub(crate) add_table_row: bool,
    pub(crate) delete_table_row: Option<usize>,
    pub(crate) reset_table_state: bool,
    pub(crate) take_firts_table_rows: Option<usize>,
    pub(crate) take_last_table_rows: Option<usize>,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            add_table_row: false,
            delete_table_row: None,
            reset_table_state: false,
            take_firts_table_rows: None,
            take_last_table_rows: None,
        }
    }
}

/// Calculation windows
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Windows {
    pub open_statistics: bool,
    pub open_settings: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            open_statistics: false,
            open_settings: false,
        }
    }
}

impl Windows {
    pub fn load(ctx: &Context, id: Id) -> Self {
        ctx.data_mut(|data| {
            data.get_persisted_mut_or_insert_with(id.with(WINDOWS), || Self::new())
                .clone()
        })
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.insert_persisted(id.with(WINDOWS), self);
        });
    }
}
