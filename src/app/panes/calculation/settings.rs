use crate::app::MAX_PRECISION;
use egui::{Grid, Id, Slider, Ui, Widget};
use serde::{Deserialize, Serialize};

use super::ID_SOURCE;

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    #[serde(skip)]
    pub(crate) resizable: bool,
    #[serde(skip)]
    pub(crate) editable: bool,
    pub(crate) label: String,
    pub(crate) precision: usize,
    pub(crate) round: u32,
    pub(crate) sticky: usize,
    pub(crate) truncate: bool,
    pub(crate) relative: bool,
    pub(crate) to_relative: bool,
    pub(crate) properties: bool,
}

impl Settings {
    pub(crate) const fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
            label: String::new(),
            precision: 2,
            round: 0,
            sticky: 0,
            truncate: false,
            relative: true,
            to_relative: false,
            properties: true,
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        let id_salt = Id::new(ID_SOURCE).with("Settings");
        Grid::new(id_salt).show(ui, |ui| {
            // Precision
            ui.label("precision");
            Slider::new(&mut self.precision, 0..=MAX_PRECISION).ui(ui);
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // Round
            ui.label("round");
            Slider::new(&mut self.round, 0..=MAX_PRECISION as _)
                .ui(ui)
                .on_hover_text("round.hover");
            ui.end_row();

            // Properties
            ui.label("properties");
            ui.checkbox(&mut self.properties, "")
                .on_hover_text("properties.hover");
            ui.end_row();

            // Relative
            ui.label("relative");
            ui.checkbox(&mut self.relative, "")
                .on_hover_text("relative.hover");
            ui.end_row();
            ui.add_enabled(self.relative, |ui: &mut Ui| {
                ui.label("to_relative");
                ui.checkbox(&mut self.to_relative, "")
                    .on_hover_text("to_relative.hover")
            });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
