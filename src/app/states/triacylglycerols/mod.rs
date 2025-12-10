use self::{settings::Settings, windows::Windows};
use egui::{Context, Id};
use serde::{Deserialize, Serialize};

pub(crate) const ID_SOURCE: &str = "Triacylglycerols";

/// State
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub reset_table_state: bool,
    pub settings: Settings,
    pub windows: Windows,
}

impl State {
    pub fn new() -> Self {
        Self {
            reset_table_state: false,
            settings: Settings::new(),
            windows: Windows::new(),
        }
    }
}

impl State {
    pub fn load(ctx: &Context, id: Id) -> Self {
        ctx.data_mut(|data| {
            data.get_persisted_mut_or_insert_with(id, || Self::new())
                .clone()
        })
    }

    pub fn remove(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.remove::<Self>(id);
        });
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.insert_persisted(id, self);
        });
    }
}

pub mod composition;
pub mod settings;

mod windows;
