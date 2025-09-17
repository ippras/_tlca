use super::ID_SOURCE;
use crate::app::{
    MAX_PRECISION,
    parameters::{Filter, Parameters, Sort, composition::COMPOSITIONS},
};
use egui::{ComboBox, Grid, Id, Key, Slider, Ui, Widget};
use serde::{Deserialize, Serialize};

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
    pub(crate) kind: Kind,

    pub(crate) parameters: Parameters,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
            precision: 2,
            percent: true,
            sticky: 0,
            truncate: true,
            kind: Kind::Absolute,
            parameters: Parameters::new(),
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

            // Composition
            ui.label("Composition");
            ComboBox::from_id_salt(ui.auto_id_with("Composition"))
                .selected_text(self.parameters.composition.text())
                .show_ui(ui, |ui| {
                    for selected_value in COMPOSITIONS {
                        ui.selectable_value(
                            &mut self.parameters.composition,
                            selected_value,
                            selected_value.text(),
                        )
                        .on_hover_ui(|ui| {
                            ui.label(selected_value.hover_text());
                        });
                    }
                })
                .response
                .on_hover_text(self.parameters.composition.hover_text());
            ui.end_row();

            // Filter
            ui.label("Filter");
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(self.parameters.filter.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.parameters.filter,
                        Filter::And,
                        Filter::And.text(),
                    )
                    .on_hover_text(Filter::And.hover_text());
                    ui.selectable_value(&mut self.parameters.filter, Filter::Or, Filter::Or.text())
                        .on_hover_text(Filter::Or.hover_text());
                    ui.selectable_value(
                        &mut self.parameters.filter,
                        Filter::Xor,
                        Filter::Xor.text(),
                    )
                    .on_hover_text(Filter::Xor.hover_text());
                })
                .response
                .on_hover_text(self.parameters.filter.hover_text());
            ui.end_row();

            // Threshold
            ui.label("Threshold");
            let number_formatter = ui.style().number_formatter.clone();
            let mut threshold = self.parameters.threshold;
            let response = Slider::new(&mut threshold, 0.0..=1.0)
                .custom_formatter(|mut value, decimals| {
                    if self.percent {
                        value *= 100.0;
                    }
                    number_formatter.format(value, decimals)
                })
                .custom_parser(|value| {
                    let mut value = value.parse().ok()?;
                    if self.percent {
                        value /= 100.0;
                    }
                    Some(value)
                })
                .logarithmic(true)
                .update_while_editing(false)
                .ui(ui);
            if response.drag_stopped()
                || (response.lost_focus() && !ui.input(|input| input.key_pressed(Key::Escape)))
            {
                self.parameters.threshold = threshold;
            }
            ui.end_row();

            // Sort
            ui.label("Sort");
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(self.parameters.sort.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.parameters.sort, Sort::Key, Sort::Key.text())
                        .on_hover_text(Sort::Key.hover_text());
                    ui.selectable_value(&mut self.parameters.sort, Sort::Value, Sort::Value.text())
                        .on_hover_text(Sort::Value.hover_text());
                })
                .response
                .on_hover_text(self.parameters.sort.hover_text());
            ui.end_row();

            // ui.separator();
            // ui.separator();
            // ui.end_row();

            // // Kind
            // ui.label("Kind");
            // ComboBox::from_id_salt(ui.auto_id_with(id_salt))
            //     .selected_text(self.kind.text())
            //     .show_ui(ui, |ui| {
            //         ui.selectable_value(&mut self.kind, Kind::Absolute, Kind::Absolute.text())
            //             .on_hover_text(Kind::Absolute.hover_text());
            //         ui.selectable_value(&mut self.kind, Kind::Difference, Kind::Difference.text())
            //             .on_hover_text(Kind::Difference.hover_text());
            //     })
            //     .response
            //     .on_hover_text(self.kind.hover_text());
            // ui.end_row();
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
    Absolute,
    Difference,
}

impl Kind {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Absolute => "Value",
            Self::Difference => "Difference",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Absolute => "Value",
            Self::Difference => "Difference",
        }
    }
}
