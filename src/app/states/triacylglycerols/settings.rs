use crate::app::{
    MAX_PRECISION,
    states::{
        fatty_acids::settings::{Filter, METRICS, Metric, SEPARATORS, Sort, Threshold},
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
use egui_l20n::prelude::*;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

const ID_SALT: LazyLock<Id> = LazyLock::new(|| Id::new(ID_SOURCE).with("Settings"));

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub percent: bool,
    pub precision: usize,
    #[serde(skip)]
    pub resizable: bool,
    pub significant: bool,
    pub standard_deviation: bool,
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
    pub filter: Filter,
    pub metric: Metric,
    pub sort: Option<Sort>,
    pub threshold: Threshold,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
            resizable: false,
            significant: false,
            standard_deviation: false,
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
            filter: Filter::Union,
            metric: Metric::HellingerDistance,
            sort: None,
            threshold: Threshold::new(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ui: &mut Ui) {
        self.precision(ui);
        self.significant(ui);
        self.percent(ui);
        self.standard_deviation(ui);
        self.truncate(ui);

        // Table
        ui.separator();
        ui.labeled_separator(ui.localize("Parameters"));
        self.composition(ui);
        self.filter(ui);
        self.sort(ui);

        ui.labeled_separator(ui.localize("Threshold"));
        self.threshold(ui);
        // self.threshold_auto(ui);
        // self.threshold_sort(ui);
        // self.threshold_filter(ui);

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

    /// Precision
    fn precision(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Precision"))
                .on_hover_localized("Precision.hover");
            Slider::new(&mut self.precision, 1..=MAX_PRECISION).ui(ui);
            if ui.button((BOOKMARK, "3")).clicked() {
                self.precision = 3;
            };
        });
    }

    // Significant
    fn significant(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Significant"))
                .on_hover_localized("Significant.hover");
            ui.checkbox(&mut self.significant, ());
        });
    }

    /// Percent
    fn percent(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Percent"))
                .on_hover_localized("Percent.hover");
            ui.checkbox(&mut self.percent, ());
        });
    }

    /// Standard deviation
    fn standard_deviation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut response = ui.label(ui.localize("StandardDeviation"));
            response |= ui.checkbox(&mut self.standard_deviation, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("StandardDeviation.hover"));
            });
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
        });
    }

    /// Threshold
    fn threshold(&mut self, ui: &mut Ui) {
        self.threshold.show(ui, self.percent);
    }

    // /// Auto threshold
    // fn threshold_auto(&mut self, ui: &mut Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(ui.localize("Threshold_Auto")).on_hover_ui(|ui| {
    //             ui.label(ui.localize("Threshold_Auto.hover"));
    //         });
    //         let number_formatter = ui.style().number_formatter.clone();
    //         let mut threshold = self.threshold.auto.0;
    //         let response = Slider::new(&mut threshold, 0.0..=1.0)
    //             .custom_formatter(|mut value, decimals| {
    //                 if self.percent {
    //                     value *= 100.0;
    //                 }
    //                 number_formatter.format(value, decimals)
    //             })
    //             .custom_parser(|value| {
    //                 let mut value = value.parse().ok()?;
    //                 if self.percent {
    //                     value /= 100.0;
    //                 }
    //                 Some(value)
    //             })
    //             .logarithmic(true)
    //             .update_while_editing(false)
    //             .ui(ui);
    //         if (response.drag_stopped() || response.lost_focus())
    //             && !ui.input(|input| input.key_pressed(Key::Escape))
    //         {
    //             self.threshold.auto.0 = threshold;
    //             self.threshold.is_auto = true;
    //         }
    //         if ui
    //             .button((BOOKMARK, if self.percent { "1.0%" } else { "0.01" }))
    //             .clicked()
    //         {
    //             self.threshold.auto.0 = 0.01;
    //             self.threshold.is_auto = true;
    //         };
    //     });
    // }

    // /// Threshold sort
    // fn threshold_sort(&mut self, ui: &mut Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(ui.localize("Threshold_Sort"))
    //             .on_hover_localized("Threshold_Sort.hover");
    //         ui.checkbox(&mut self.threshold.sort, ());
    //     });
    // }

    // /// Threshold filter
    // fn threshold_filter(&mut self, ui: &mut Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(ui.localize("Threshold_Filter"))
    //             .on_hover_localized("Threshold_Filter.hover");
    //         ui.checkbox(&mut self.threshold.filter, ());
    //     });
    // }

    /// Sort
    fn sort(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Sort")).on_hover_ui(|ui| {
                ui.label(ui.localize("Sort.hover"));
            });
            let mut checked = self.sort.is_some();
            if ui.checkbox(&mut checked, ()).changed() {
                self.sort = if checked { Some(Sort::Key) } else { None };
            }
            ui.add_enabled_ui(checked, |ui| {
                let text = match self.sort {
                    Some(sort) => WidgetText::from(ui.localize(sort.text())),
                    None => WidgetText::from(""),
                };
                let response = ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                    .selected_text(text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.sort,
                            Some(Sort::Key),
                            ui.localize(Sort::Key.text()),
                        )
                        .on_hover_text(ui.localize(Sort::Key.hover_text()));
                        ui.selectable_value(
                            &mut self.sort,
                            Some(Sort::Value),
                            ui.localize(Sort::Value.text()),
                        )
                        .on_hover_text(ui.localize(Sort::Value.hover_text()));
                    })
                    .response;
                if let Some(sort) = self.sort {
                    response.on_hover_localized(sort.hover_text());
                }
            });
        });
    }

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
