use crate::app::MAX_PRECISION;
use egui::{ComboBox, Grid, Id, Slider, Ui, Widget};
use serde::{Deserialize, Serialize};

use super::ID_SOURCE;

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    #[serde(skip)]
    pub(crate) resizable: bool,
    #[serde(skip)]
    pub(crate) editable: bool,
    pub(crate) precision: usize,
    pub(crate) percent: bool,
    pub(crate) sticky: usize,
    pub(crate) truncate: bool,
    pub(crate) properties: bool,
    pub(crate) kind: Kind,
    pub(crate) view: View,
}

impl Settings {
    pub(crate) const fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
            precision: 2,
            percent: true,
            sticky: 0,
            truncate: true,
            properties: true,
            kind: Kind::Value,
            view: View::Data,
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        let id_salt = Id::new(ID_SOURCE).with("Settings");
        Grid::new(id_salt).show(ui, |ui| {
            // Precision
            ui.label("precision");
            Slider::new(&mut self.precision, 0..=MAX_PRECISION).ui(ui);
            ui.end_row();

            // Percent
            let mut response = ui.label("percent");
            response |= ui.checkbox(&mut self.percent, "");
            response.on_hover_ui(|ui| {
                ui.label("percent.hover");
            });
            ui.end_row();

            // Truncate
            let mut response = ui.label("truncate");
            response |= ui.checkbox(&mut self.truncate, "");
            response.on_hover_ui(|ui| {
                ui.label("truncate.hover");
            });
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // Properties
            ui.label("properties");
            ui.checkbox(&mut self.properties, "")
                .on_hover_text("properties.hover");
            ui.end_row();

            // Kind
            ui.label("Kind");
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(self.kind.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.kind, Kind::Value, Kind::Value.text())
                        .on_hover_text(Kind::Value.hover_text());
                    ui.selectable_value(&mut self.kind, Kind::Difference, Kind::Difference.text())
                        .on_hover_text(Kind::Difference.hover_text());
                })
                .response
                .on_hover_text(self.kind.hover_text());
            ui.end_row();

            // Statistics
            ui.label("statistics");
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(self.view.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.view, View::Data, View::Data.text())
                        .on_hover_text(View::Data.hover_text());
                    ui.selectable_value(&mut self.view, View::Statistics, View::Statistics.text())
                        .on_hover_text(View::Statistics.hover_text());
                })
                .response
                .on_hover_text(self.kind.hover_text());
            ui.end_row();
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Kind {
    #[default]
    Value,
    Difference,
}

impl Kind {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Value => "Value",
            Self::Difference => "Difference",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Value => "Value",
            Self::Difference => "Difference",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum View {
    #[default]
    Data,
    Statistics,
}

impl View {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Data => "Data",
            Self::Statistics => "Statistics",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Data => "Data",
            Self::Statistics => "Statistics",
        }
    }
}
