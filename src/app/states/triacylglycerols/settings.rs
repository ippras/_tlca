use crate::app::{
    MAX_PRECISION,
    states::{
        fatty_acids::settings::{Join, METRICS, Metric, SEPARATORS},
        triacylglycerols::{
            ID_SOURCE,
            composition::{
                COMPOSITIONS, Composition, SPECIES_MONO, SPECIES_POSITIONAL, SPECIES_STEREO,
                TYPE_MONO, TYPE_POSITIONAL, UNSATURATION_MONO,
            },
        },
    },
};
use egui::{
    ComboBox, Id, Key, KeyboardShortcut, Modifiers, RichText, Slider, Ui, Widget, WidgetText,
};
use egui_ext::LabeledSeparator;
#[cfg(feature = "markdown")]
use egui_ext::Markdown;
use egui_l10n::prelude::*;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use widgets::settings::{Major, Mean, Precision, Sort};

const ID_SALT: LazyLock<Id> = LazyLock::new(|| Id::new(ID_SOURCE).with("Settings"));

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub precision: Precision,
    pub mean: Mean,
    pub major: Major,

    #[serde(skip)]
    pub resizable: bool,
    pub truncate: bool,

    // Table settings
    #[serde(skip)]
    pub edit: bool,
    pub sticky: usize,
    // Metrics settings
    pub chaddock: bool,
    // Moment settings
    pub bias: bool,
    //
    pub composition: Composition,
    pub filter: Join,
    pub metric: Metric,
    pub sort: Sort,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            precision: Precision::new(),
            mean: Mean::new(),
            major: Major::new(),

            resizable: false,
            truncate: true,

            // Table settings
            edit: false,
            sticky: 0,
            // Metrics settings
            chaddock: true,
            // Moment settings
            bias: true,
            //
            composition: SPECIES_STEREO,
            filter: Join::Union,
            metric: Metric::HellingerDistance,
            sort: Sort::new(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.precision.show(ui);
        });

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.mean.show(ui);
        });

        self.truncate(ui);

        // Table
        ui.separator();
        ui.labeled_separator(ui.localize("Parameters"));
        self.composition(ui);
        self.filter(ui);
        self.sort.show(ui);

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.major.show(ui, &[], self.precision.percent);
        });

        // Metrics
        ui.collapsing(ui.localize("Metric?PluralCategory=other"), |ui| {
            self.metric(ui);
            self.chaddock(ui);
        });

        // Moments
        ui.collapsing(ui.localize("Moments"), |ui| {
            self.bias(ui);
        });
    }

    /// Truncate
    fn truncate(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut response = ui.label(ui.localize("Truncate"));
            response |= ui.checkbox(&mut self.truncate, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Truncate.hover"));
            });
        });
    }

    /// Composition
    fn composition(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
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
            ui.menu_button(BOOKMARK, |ui| {
                if ui.button((BOOKMARK, "PSC")).clicked() {
                    self.composition = SPECIES_POSITIONAL;
                };
                if ui.button((BOOKMARK, "MSC")).clicked() {
                    self.composition = SPECIES_MONO;
                };
                if ui.button((BOOKMARK, "PTC")).clicked() {
                    self.composition = TYPE_POSITIONAL;
                };
                if ui.button((BOOKMARK, "MTC")).clicked() {
                    self.composition = TYPE_MONO;
                };
                if ui.button((BOOKMARK, "MUC")).clicked() {
                    self.composition = UNSATURATION_MONO;
                };
            });
            // if ui.input_mut(|input| {
            //     input.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::ArrowDown))
            // }) {
            //     self.composition = self.composition.forward();
            // }
            // if ui.input_mut(|input| {
            //     input.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::ArrowUp))
            // }) {
            //     self.composition = self.composition.backward();
            // }
        });
    }

    /// Filter
    fn filter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Filter")).on_hover_ui(|ui| {
                ui.label(ui.localize("Filter.hover"));
            });
            ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                .selected_text(ui.localize(self.filter.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.filter,
                        Join::Intersection,
                        (
                            Join::Intersection.icon(),
                            ui.localize(Join::Intersection.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Join::Intersection.hover_text()));
                    ui.selectable_value(
                        &mut self.filter,
                        Join::Union,
                        (Join::Union.icon(), ui.localize(Join::Union.text())),
                    )
                    .on_hover_text(ui.localize(Join::Union.hover_text()));
                    ui.selectable_value(
                        &mut self.filter,
                        Join::Difference,
                        (
                            Join::Difference.icon(),
                            ui.localize(Join::Difference.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Join::Difference.hover_text()));
                })
                .response
                .on_hover_text(RichText::new(self.filter.icon()).heading());
        });
    }

    // /// Sort
    // fn sort(&mut self, ui: &mut Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(ui.localize("Sort")).on_hover_ui(|ui| {
    //             ui.label(ui.localize("Sort.hover"));
    //         });
    //         let mut checked = self.sort.is_some();
    //         if ui.checkbox(&mut checked, ()).changed() {
    //             self.sort = if checked { Some(Sort::Key) } else { None };
    //         }
    //         ui.add_enabled_ui(checked, |ui| {
    //             let text = match self.sort {
    //                 Some(sort) => WidgetText::from(ui.localize(sort.text())),
    //                 None => WidgetText::from(""),
    //             };
    //             let response = ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
    //                 .selected_text(text)
    //                 .show_ui(ui, |ui| {
    //                     ui.selectable_value(
    //                         &mut self.sort,
    //                         Some(Sort::Key),
    //                         ui.localize(Sort::Key.text()),
    //                     )
    //                     .on_hover_text(ui.localize(Sort::Key.hover_text()));
    //                     ui.selectable_value(
    //                         &mut self.sort,
    //                         Some(Sort::Value),
    //                         ui.localize(Sort::Value.text()),
    //                     )
    //                     .on_hover_text(ui.localize(Sort::Value.hover_text()));
    //                 })
    //                 .response;
    //             if let Some(sort) = self.sort {
    //                 response.on_hover_ui(|ui| {
    //                     ui.label(ui.localize(sort.hover_text()));
    //                 });
    //             }
    //         });
    //     });
    // }

    /// Metric
    fn metric(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Metric?PluralCategory=one"))
                .on_hover_text(ui.localize("Metric.hover"));
            #[allow(unused_variables)]
            let response = ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
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
        });
    }

    /// Chaddock
    fn chaddock(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut response = ui.label(ui.localize("Chaddock"));
            response |= ui.checkbox(&mut self.chaddock, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Chaddock.hover"));
            });
        });
    }

    /// Bias
    fn bias(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut response = ui.label(ui.localize("Bias"));
            response |= ui.checkbox(&mut self.bias, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Bias.hover"));
            });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
