use crate::app::{
    MAX_PRECISION,
    states::{
        Filter, METRICS, Metric, SEPARATORS, Sort,
        triacylglycerols::{
            ID_SOURCE,
            composition::{COMPOSITIONS, Composition, SPECIES_STEREO},
        },
    },
};
use egui::{ComboBox, Grid, Id, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui, Widget};
use egui_ext::LabeledSeparator;
#[cfg(feature = "markdown")]
use egui_ext::Markdown;
use egui_l20n::prelude::*;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub percent: bool,
    pub precision: usize,
    #[serde(skip)]
    pub resizable: bool,
    pub truncate: bool,
    // Table settings
    #[serde(skip)]
    pub editable: bool,
    pub sticky: usize,
    // Metrics settings
    pub chaddock: bool,
    // Moment settings
    pub bias: bool,

    //
    pub composition: Composition,
    pub filter: Filter,
    pub threshold: OrderedFloat<f64>,
    pub sort: Sort,
    pub metric: Metric,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            percent: true,
            precision: 2,
            resizable: false,
            truncate: true,

            editable: false,
            sticky: 0,

            chaddock: true,

            bias: true,

            //
            composition: SPECIES_STEREO,
            filter: Filter::Union,
            threshold: OrderedFloat(0.0),
            sort: Sort::Value,
            metric: Metric::HellingerDistance,
        }
    }
}

impl Settings {
    pub fn show(&mut self, ui: &mut Ui) {
        let id_salt = Id::new(ID_SOURCE).with("Settings");
        Grid::new(id_salt).show(ui, |ui| {
            // Precision
            ui.label(ui.localize("Precision")).on_hover_ui(|ui| {
                ui.label(ui.localize("Precision.hover"));
            });
            Slider::new(&mut self.precision, 1..=MAX_PRECISION).ui(ui);
            ui.end_row();

            // Percent
            let mut response = ui.label(ui.localize("Percent"));
            response |= ui.checkbox(&mut self.percent, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Percent.hover"));
            });
            ui.end_row();

            // Truncate
            let mut response = ui.label(ui.localize("Truncate"));
            response |= ui.checkbox(&mut self.truncate, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Truncate.hover"));
            });
            ui.end_row();

            ui.separator();
            ui.labeled_separator(ui.localize("Parameters"));
            ui.end_row();

            // Composition
            ui.label(ui.localize("Composition")).on_hover_ui(|ui| {
                ui.label(ui.localize("Composition.hover"));
            });
            ComboBox::from_id_salt(ui.auto_id_with("Composition"))
                .selected_text(ui.localize(self.composition.text()))
                .show_ui(ui, |ui| {
                    for selected_value in COMPOSITIONS {
                        ui.selectable_value(
                            &mut self.composition,
                            selected_value,
                            ui.localize(selected_value.text()),
                        )
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(selected_value.hover_text()));
                        });
                    }
                })
                .response
                .on_hover_text(ui.localize(self.composition.hover_text()));
            if ui.input_mut(|input| {
                input.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::ArrowDown))
            }) {
                self.composition = self.composition.forward();
            }
            if ui.input_mut(|input| {
                input.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::ArrowUp))
            }) {
                self.composition = self.composition.backward();
            }
            ui.end_row();

            // Filter
            ui.label(ui.localize("Filter")).on_hover_ui(|ui| {
                ui.label(ui.localize("Filter.hover"));
            });
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(ui.localize(self.filter.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.filter,
                        Filter::Intersection,
                        (
                            Filter::Intersection.icon(),
                            ui.localize(Filter::Intersection.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Filter::Intersection.hover_text()));
                    ui.selectable_value(
                        &mut self.filter,
                        Filter::Union,
                        (Filter::Union.icon(), ui.localize(Filter::Union.text())),
                    )
                    .on_hover_text(ui.localize(Filter::Union.hover_text()));
                    ui.selectable_value(
                        &mut self.filter,
                        Filter::Difference,
                        (
                            Filter::Difference.icon(),
                            ui.localize(Filter::Difference.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Filter::Difference.hover_text()));
                })
                .response
                .on_hover_text(RichText::new(self.filter.icon()).heading());
            ui.end_row();

            // Threshold
            ui.label(ui.localize("Threshold")).on_hover_ui(|ui| {
                ui.label(ui.localize("Threshold.hover"));
            });
            let number_formatter = ui.style().number_formatter.clone();
            let mut threshold = self.threshold;
            let response = Slider::new(&mut threshold.0, 0.0..=1.0)
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
            if (response.drag_stopped() || response.lost_focus())
                && !ui.input(|input| input.key_pressed(Key::Escape))
            {
                self.threshold = threshold;
            }
            ui.end_row();

            // Sort
            ui.label(ui.localize("Sort")).on_hover_ui(|ui| {
                ui.label(ui.localize("Sort.hover"));
            });
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(ui.localize(self.sort.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort, Sort::Key, ui.localize(Sort::Key.text()))
                        .on_hover_text(ui.localize(Sort::Key.hover_text()));
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::Value,
                        ui.localize(Sort::Value.text()),
                    )
                    .on_hover_text(ui.localize(Sort::Value.hover_text()));
                })
                .response
                .on_hover_text(ui.localize(self.sort.hover_text()));
            ui.end_row();

            ui.separator();
            ui.labeled_separator(ui.localize("Metric?PluralCategory=other"));
            ui.end_row();

            // Metric
            ui.label(ui.localize("Metric?PluralCategory=one"))
                .on_hover_text(ui.localize("Metric.hover"));
            #[allow(unused_variables)]
            let response = ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(ui.localize(self.metric.text()))
                .show_ui(ui, |ui| {
                    for (index, metric) in METRICS.into_iter().enumerate() {
                        if SEPARATORS.contains(&index) {
                            ui.separator();
                        }
                        #[allow(unused_variables)]
                        let response = ui.selectable_value(
                            &mut self.metric,
                            metric,
                            ui.localize(metric.text()),
                        );
                        #[cfg(feature = "markdown")]
                        response.on_hover_ui(|ui| {
                            ui.markdown(metric.hover_markdown());
                        });
                    }
                })
                .response;
            #[cfg(feature = "markdown")]
            response.on_hover_ui(|ui| {
                ui.markdown(self.metric.hover_markdown());
            });
            ui.end_row();

            // Chaddock
            let mut response = ui.label(ui.localize("Chaddock"));
            response |= ui.checkbox(&mut self.chaddock, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Chaddock.hover"));
            });
            ui.end_row();

            // Moments
            ui.separator();
            ui.labeled_separator(ui.localize("Moments"));
            ui.end_row();

            // Bias
            let mut response = ui.label(ui.localize("Bias"));
            response |= ui.checkbox(&mut self.bias, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Bias.hover"));
            });
            ui.end_row();
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameters {}

impl Parameters {
    pub fn new() -> Self {
        Self {}
    }
}

// impl Default for Parameters {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Hash for Parameters {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.composition.hash(state);
//         self.filter.hash(state);
//         self.threshold.ord().hash(state);
//         self.sort.hash(state);
//         self.metric.hash(state);
//     }
// }
