use crate::{app::states::fatty_acids::ID_SOURCE, r#const::markdown::*};
use const_format::formatcp;
use egui::{
    ComboBox, Id, Key, Popup, PopupCloseBehavior, RichText, Slider, Ui, Widget, WidgetText,
};
use egui_dnd::dnd;
use egui_ext::LabeledSeparator;
#[cfg(feature = "markdown")]
use egui_ext::Markdown;
use egui_l10n::prelude::*;
use egui_phosphor::regular::{BOOKMARK, DOTS_SIX_VERTICAL, EXCLUDE, INTERSECT, UNITE};
use fatty_acid_expressions::r#const::{EXPRESSION, PREFIX as FAE};
use lipid::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    sync::LazyLock,
};
use widgets::{
    Show,
    fatty_acids::settings::Expressions,
    settings::{HighlightSortFilter, Major, Mean, Precision, Sort, ThresholdZero},
};

pub(crate) const METRICS: [Metric; 9] = [
    Metric::HellingerDistance,
    Metric::JensenShannonDistance,
    Metric::BhattacharyyaDistance,
    //
    Metric::CosineDistance,
    Metric::JaccardDistance,
    Metric::OverlapDistance,
    //
    Metric::EuclideanDistance,
    Metric::ChebyshevDistance,
    Metric::ManhattanDistance,
];

pub(crate) const SEPARATORS: [usize; 2] = [3, 6];

const ID_SALT: LazyLock<Id> = LazyLock::new(|| Id::new(ID_SOURCE).with("Settings"));

const STEREOSPECIFIC_NUMBERS: [StereospecificNumbers; 3] = [
    StereospecificNumbers::Sn123,
    StereospecificNumbers::Sn13,
    StereospecificNumbers::Sn2,
];

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    // Display
    pub(crate) msd: Mean,
    pub(crate) precision: Precision,
    pub(crate) major: Major,

    #[serde(skip)]
    pub(crate) resizable: bool,
    pub(crate) truncate: bool,
    pub(crate) sticky_columns: usize,
    // Table settings
    #[serde(skip)]
    pub(crate) editable: bool,
    // Factors settings
    pub(crate) factor: Factor,
    pub(crate) normalize_factor: bool,
    // Metrics settings
    pub(crate) chaddock: bool,
    pub(crate) metric: Metric,
    //
    pub(crate) join: Join,
    pub(crate) sort: Sort,

    pub(crate) stereospecific_numbers: StereospecificNumbers,

    // Expressions settings
    pub(crate) expressions: Expressions,
    pub(crate) highlight_sort_filter: HighlightSortFilter,

    pub(crate) reset: bool,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {
            // Display
            precision: Precision::new(),
            msd: Mean::new(),
            major: Major::builder().bookmark(0.01).build(),

            resizable: false,
            truncate: true,
            // Table settings
            editable: false,
            sticky_columns: 0,
            // Factors settings
            factor: Factor::Enrichment,
            normalize_factor: false,
            // Metrics settings
            chaddock: true,
            metric: Metric::HellingerDistance,

            // Expressions settings
            stereospecific_numbers: StereospecificNumbers::Sn123,
            join: Join::Union,
            sort: Sort::new(),

            expressions: Expressions::new(),
            highlight_sort_filter: HighlightSortFilter::new(),

            reset: false,
        }
    }
}

impl Settings {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.precision.show(ui);
        });

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.msd.show(ui);
        });

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.major.show(ui, &[], self.precision.percent);
        });

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            self.sort.show(ui);
        });

        self.truncate(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Parameters"));

        self.stereospecific_numbers(ui);
        self.join(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Factor?PluralCategory=other"));

        self.factors(ui);

        // Metrics
        ui.collapsing(
            RichText::new(ui.localize("Metric?PluralCategory=other")).heading(),
            |ui| {
                self.metrics(ui);
            },
        );

        // Expressions
        ui.collapsing(
            RichText::new(ui.localize(formatcp!("{FAE}_{EXPRESSION}?PluralCategory=other")))
                .heading(),
            |ui| {
                self.expressions.show(ui);
                ui.separator();
                ui.horizontal(|ui| {
                    // ui.label(ui.localize("Predicate"));
                    // ui.label(ui.localize("Zero"));
                    ui.label("Predicate");
                    ui.label("Non zero");
                });
                self.highlight_sort_filter.show(ui);
            },
        );
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

    /// Stereospecific numbers
    fn stereospecific_numbers(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("StereospecificNumber?number=many"))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize("StereospecificNumber.abbreviation?number=other"));
                });
            ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                .selected_text(ui.localize(self.stereospecific_numbers.text()))
                .show_ui(ui, |ui| {
                    for stereospecific_number in STEREOSPECIFIC_NUMBERS {
                        ui.selectable_value(
                            &mut self.stereospecific_numbers,
                            stereospecific_number,
                            ui.localize(stereospecific_number.text()),
                        )
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(stereospecific_number.hover_text()));
                        });
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.stereospecific_numbers.hover_text()));
                });
        });
    }

    /// Join
    fn join(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Filter")).on_hover_ui(|ui| {
                ui.label(ui.localize("Filter.hover"));
            });
            ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                .selected_text(ui.localize(self.join.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.join,
                        Join::Intersection,
                        (
                            Join::Intersection.icon(),
                            ui.localize(Join::Intersection.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Join::Intersection.hover_text()));
                    ui.selectable_value(
                        &mut self.join,
                        Join::Union,
                        (Join::Union.icon(), ui.localize(Join::Union.text())),
                    )
                    .on_hover_text(ui.localize(Join::Union.hover_text()));
                    ui.selectable_value(
                        &mut self.join,
                        Join::Difference,
                        (
                            Join::Difference.icon(),
                            ui.localize(Join::Difference.text()),
                        ),
                    )
                    .on_hover_text(ui.localize(Join::Difference.hover_text()));
                })
                .response
                .on_hover_text(RichText::new(self.join.icon()).heading());
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

    /// Factors
    fn factors(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Factor?Number=many"))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize("Factor.abbreviation?Number=other"));
                });
            ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                .selected_text(ui.localize(self.factor.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.factor,
                        Factor::Enrichment,
                        ui.localize(Factor::Enrichment.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(Factor::Enrichment.hover_text()));
                    });
                    ui.selectable_value(
                        &mut self.factor,
                        Factor::Selectivity,
                        ui.localize(Factor::Selectivity.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(Factor::Selectivity.hover_text()));
                    });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.factor.hover_text()));
                });
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize("NormalizeFactor")).on_hover_ui(|ui| {
                ui.label(ui.localize("NormalizeFactor.hover"));
            });
            ui.checkbox(&mut self.normalize_factor, ());
        });
    }

    /// Metric
    fn metrics(&mut self, ui: &mut Ui) {
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

        // Chaddock
        ui.horizontal(|ui| {
            let mut response = ui.label(ui.localize("Chaddock"));
            response |= ui.checkbox(&mut self.chaddock, "");
            response.on_hover_ui(|ui| {
                ui.label(ui.localize("Chaddock.hover"));
            });
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// Stereospecific numbers
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum StereospecificNumbers {
    Sn123,
    Sn13,
    Sn2,
}

impl StereospecificNumbers {
    pub(crate) fn id(&self) -> &'static str {
        match self {
            StereospecificNumbers::Sn123 => STEREOSPECIFIC_NUMBERS123,
            StereospecificNumbers::Sn13 => STEREOSPECIFIC_NUMBERS13,
            StereospecificNumbers::Sn2 => STEREOSPECIFIC_NUMBERS2,
        }
    }
}

impl StereospecificNumbers {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Sn123 => "StereospecificNumber.abbreviation?number=123",
            Self::Sn13 => "StereospecificNumber.abbreviation?number=13",
            Self::Sn2 => "StereospecificNumber.abbreviation?number=2",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Sn123 => "StereospecificNumber?number=123",
            Self::Sn13 => "StereospecificNumber?number=13",
            Self::Sn2 => "StereospecificNumber?number=2",
        }
    }
}

/// Stereospecific numbers
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Factor {
    Selectivity,
    Enrichment,
}

impl Factor {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Enrichment => "EnrichmentFactor",
            Self::Selectivity => "SelectivityFactor",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Enrichment => "EnrichmentFactor.hover",
            Self::Selectivity => "SelectivityFactor.hover",
        }
    }
}

/// Join
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Join {
    #[default]
    Intersection, // And
    Union,      // Or
    Difference, // Xor
}

impl Join {
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            Self::Intersection => INTERSECT,
            Self::Union => UNITE,
            Self::Difference => EXCLUDE,
        }
    }

    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Intersection => "Filter_Intersection",
            Self::Union => "Filter_Union",
            Self::Difference => "Filter_Difference",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Intersection => "Filter_Intersection.hover",
            Self::Union => "Filter_Union.hover",
            Self::Difference => "Filter_Difference.hover",
        }
    }
}

/// Metric
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Metric {
    // Distance between two discrete probability distributions
    HellingerDistance,
    JensenShannonDistance,
    BhattacharyyaDistance,
    // Distance between two points
    EuclideanDistance,
    ChebyshevDistance,
    ManhattanDistance,
    // Distance between two series
    CosineDistance,
    JaccardDistance,
    OverlapDistance,
}

impl Metric {
    pub(crate) fn is_finite(&self) -> bool {
        matches!(
            self,
            Metric::HellingerDistance
                | Metric::JensenShannonDistance
                | Metric::CosineDistance
                | Metric::JaccardDistance
                | Metric::OverlapDistance
        )
    }
}

impl Metric {
    pub(crate) fn forward(&self) -> Self {
        match self {
            Self::HellingerDistance => Self::JensenShannonDistance,
            Self::JensenShannonDistance => Self::BhattacharyyaDistance,
            Self::BhattacharyyaDistance => Self::EuclideanDistance,
            Self::EuclideanDistance => Self::ChebyshevDistance,
            Self::ChebyshevDistance => Self::ManhattanDistance,
            Self::ManhattanDistance => Self::CosineDistance,
            Self::CosineDistance => Self::JaccardDistance,
            Self::JaccardDistance => Self::OverlapDistance,
            Self::OverlapDistance => Self::OverlapDistance,
        }
    }

    pub(crate) fn backward(&self) -> Self {
        match self {
            Self::HellingerDistance => Self::HellingerDistance,
            Self::JensenShannonDistance => Self::HellingerDistance,
            Self::BhattacharyyaDistance => Self::JensenShannonDistance,
            Self::EuclideanDistance => Self::BhattacharyyaDistance,
            Self::ChebyshevDistance => Self::EuclideanDistance,
            Self::ManhattanDistance => Self::ChebyshevDistance,
            Self::CosineDistance => Self::ManhattanDistance,
            Self::JaccardDistance => Self::CosineDistance,
            Self::OverlapDistance => Self::JaccardDistance,
        }
    }
}

impl Metric {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::HellingerDistance => "HellingerDistance",
            Self::JensenShannonDistance => "JensenShannonDistance",
            Self::BhattacharyyaDistance => "BhattacharyyaDistance",
            Self::EuclideanDistance => "EuclideanDistance",
            Self::ChebyshevDistance => "ChebyshevDistance",
            Self::ManhattanDistance => "ManhattanDistance",
            Self::CosineDistance => "CosineDistance",
            Self::JaccardDistance => "JaccardDistance",
            Self::OverlapDistance => "OverlapDistance",
        }
    }

    pub(crate) fn hover_markdown(&self) -> &'static str {
        match self {
            Self::HellingerDistance => HELLINGER_COEFFICIENT,
            Self::JensenShannonDistance => JENSEN_SHANNON_COEFFICIENT,
            Self::BhattacharyyaDistance => BHATTACHARYYA_COEFFICIENT,
            Self::EuclideanDistance => EUCLIDEAN_DISTANCE,
            Self::ChebyshevDistance => CHEBYSHEV_DISTANCE,
            Self::ManhattanDistance => MANHATTAN_DISTANCE,
            Self::CosineDistance => COSINE_COEFFICIENT,
            Self::JaccardDistance => JACCARD_COEFFICIENT,
            Self::OverlapDistance => OVERLAP_COEFFICIENT,
        }
    }
}
