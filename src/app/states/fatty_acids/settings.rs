use crate::{
    app::{MAX_PRECISION, states::fatty_acids::ID_SOURCE},
    r#const::markdown::*,
};
use egui::{
    ComboBox, Id, Key, Popup, PopupCloseBehavior, RichText, Slider, Ui, Widget, WidgetText,
};
use egui_dnd::dnd;
use egui_ext::LabeledSeparator;
#[cfg(feature = "markdown")]
use egui_ext::Markdown;
use egui_l20n::prelude::*;
use egui_phosphor::regular::{BOOKMARK, DOTS_SIX_VERTICAL, EXCLUDE, INTERSECT, UNITE};
use lipid::prelude::*;
use ordered_float::OrderedFloat;
use polars_utils::format_list_truncated;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    sync::LazyLock,
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
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    #[serde(skip)]
    pub(crate) resizable: bool,
    pub(crate) significant: bool,
    pub(crate) standard_deviation: bool,
    pub(crate) truncate: bool,
    // Table settings
    #[serde(skip)]
    pub(crate) editable: bool,
    pub(crate) sticky: usize,
    // Factors settings
    pub(crate) factor: Factor,
    pub(crate) normalize_factor: bool,
    // Metrics settings
    pub(crate) chaddock: bool,
    pub(crate) metric: Metric,
    // Indices settings
    pub(crate) indices: Indices,
    //
    pub(crate) filter: Filter,
    pub(crate) sort: Option<Sort>,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: Threshold,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
            resizable: false,
            significant: false,
            standard_deviation: false,
            truncate: true,
            // Table settings
            editable: false,
            sticky: 0,
            // Factors settings
            factor: Factor::Enrichment,
            normalize_factor: false,
            // Metrics settings
            chaddock: true,
            metric: Metric::HellingerDistance,
            // Indices settings
            indices: Indices::new(),

            stereospecific_numbers: StereospecificNumbers::Sn123,
            filter: Filter::Union,
            threshold: Threshold::new(),
            sort: None,
        }
    }
}

impl Settings {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        self.precision(ui);
        self.significant(ui);
        self.percent(ui);
        self.standard_deviation(ui);
        self.truncate(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Parameters"));

        self.stereospecific_numbers(ui);
        self.filter(ui);

        self.sort(ui);

        ui.labeled_separator(ui.localize("Threshold"));

        self.threshold_auto(ui);
        self.threshold_sort(ui);
        self.threshold_filter(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Factor?PluralCategory=other"));

        self.factors(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Metric?PluralCategory=other"));

        self.metrics(ui);

        ui.separator();
        ui.labeled_separator(ui.localize("Indices"));

        self.indices(ui);
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

    /// Auto threshold
    fn threshold_auto(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Auto")).on_hover_ui(|ui| {
                ui.label(ui.localize("Threshold_Auto.hover"));
            });
            let number_formatter = ui.style().number_formatter.clone();
            let mut threshold = self.threshold.auto.0;
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
            if (response.drag_stopped() || response.lost_focus())
                && !ui.input(|input| input.key_pressed(Key::Escape))
            {
                self.threshold.auto.0 = threshold;
                self.threshold.is_auto = true;
            }
            if ui
                .button((BOOKMARK, if self.percent { "1.0%" } else { "0.01" }))
                .clicked()
            {
                self.threshold.auto.0 = 0.01;
                self.threshold.is_auto = true;
            };
        });
    }

    /// Threshold sort
    fn threshold_sort(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Sort"))
                .on_hover_localized("Threshold_Sort.hover");
            ui.checkbox(&mut self.threshold.sort, ());
        });
    }

    /// Threshold filter
    fn threshold_filter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Filter"))
                .on_hover_localized("Threshold_Filter.hover");
            ui.checkbox(&mut self.threshold.filter, ());
        });
    }

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
            ui.label(ui.localize("NormalizeFactor"))
                .on_hover_localized("NormalizeFactor.hover");
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

    /// Indices
    fn indices(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Indices")).on_hover_ui(|ui| {
                ui.label(ui.localize("Indices.hover"));
            });
            let selected_text = format_list_truncated!(
                self.indices
                    .0
                    .iter()
                    .filter(|index| index.visible)
                    .map(|index| ui.localize(&format!("Indices_{}", index.name))),
                1
            );
            ComboBox::from_id_salt(ui.auto_id_with(*ID_SALT))
                .selected_text(selected_text)
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .show_ui(ui, |ui| self.indices.show(ui));
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

/// Indices
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Indices(Vec<Index>);

impl Indices {
    pub(crate) fn new() -> Self {
        Self(vec![
            Index::new("Saturated"),
            Index::new("Monounsaturated"),
            Index::new("Polyunsaturated"),
            Index::new("Unsaturated"),
            Index::new("Unsaturated-9"),
            Index::new("Unsaturated-6"),
            Index::new("Unsaturated-3"),
            Index::new("Unsaturated9"),
            Index::new("Trans"),
            Index::new("EicosapentaenoicAndDocosahexaenoic"),
            Index::new("FishLipidQuality"),
            Index::new("HealthPromotingIndex"),
            Index::new("HypocholesterolemicToHypercholesterolemic"),
            Index::new("IndexOfAtherogenicity"),
            Index::new("IndexOfThrombogenicity"),
            Index::new("LinoleicToAlphaLinolenic"),
            Index::new("Polyunsaturated-6ToPolyunsaturated-3"),
            Index::new("PolyunsaturatedToSaturated"),
            Index::new("UnsaturationIndex"),
        ])
    }
}

impl Deref for Indices {
    type Target = Vec<Index>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Indices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Indices {
    fn show(&mut self, ui: &mut Ui) {
        let mut visible_all = None;
        let response = dnd(ui, ui.auto_id_with("Indices")).show(
            self.iter_mut(),
            |ui, index, handle, _state| {
                ui.horizontal(|ui| {
                    let visible = index.visible;
                    handle.ui(ui, |ui| {
                        ui.label(DOTS_SIX_VERTICAL);
                    });
                    ui.checkbox(&mut index.visible, "");
                    let mut label = RichText::new(&index.name);
                    if !visible {
                        label = label.weak();
                    }
                    let response = ui.label(label);
                    Popup::context_menu(&response)
                        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                        .show(|ui| {
                            if ui.button("Show all").clicked() {
                                visible_all = Some(true);
                            }
                            if ui.button("Hide all").clicked() {
                                visible_all = Some(false);
                            }
                        });
                });
            },
        );
        if response.is_drag_finished() {
            response.update_vec(self.as_mut_slice());
        }
        if let Some(visible) = visible_all {
            for index in &mut self.0 {
                index.visible = visible;
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Index {
    pub(crate) name: String,
    pub(crate) visible: bool,
}

impl Index {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            visible: true,
        }
    }
}

/// Filter
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Filter {
    #[default]
    Intersection, // And
    Union,      // Or
    Difference, // Xor
}

impl Filter {
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

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Sort {
    Key,
    Value,
}

impl Sort {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Key => "Sort_Key",
            Self::Value => "Sort_Value",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Key => "Sort_Key.hover",
            Self::Value => "Sort_Value.hover",
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

/// Threshold
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Threshold {
    pub(crate) auto: OrderedFloat<f64>,
    pub(crate) filter: bool,
    pub(crate) is_auto: bool,
    pub(crate) manual: Vec<bool>,
    pub(crate) sort: bool,
}

impl Threshold {
    pub(crate) fn new() -> Self {
        Self {
            auto: OrderedFloat(0.0),
            filter: false,
            is_auto: true,
            manual: Vec::new(),
            sort: false,
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui, percent: bool) {
        self.is_auto(ui);
        self.auto(ui, percent);
        self.sort(ui);
        self.filter(ui);
    }

    /// Is auto threshold
    fn is_auto(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_IsAuto"))
                .on_hover_localized("Threshold_IsAuto.hover");
            ui.checkbox(&mut self.is_auto, ());
        });
    }

    /// Auto threshold
    fn auto(&mut self, ui: &mut Ui, percent: bool) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Auto")).on_hover_ui(|ui| {
                ui.label(ui.localize("Threshold_Auto.hover"));
            });
            let number_formatter = ui.style().number_formatter.clone();
            let mut threshold = self.auto.0;
            let response = Slider::new(&mut threshold, 0.0..=1.0)
                .custom_formatter(|mut value, decimals| {
                    if percent {
                        value *= 100.0;
                    }
                    number_formatter.format(value, decimals)
                })
                .custom_parser(|value| {
                    let mut parsed = value.parse().ok()?;
                    if percent {
                        parsed /= 100.0;
                    }
                    Some(parsed)
                })
                .logarithmic(true)
                .update_while_editing(false)
                .ui(ui);
            if (response.drag_stopped() || response.lost_focus())
                && !ui.input(|input| input.key_pressed(Key::Escape))
            {
                self.auto.0 = threshold;
                self.is_auto = true;
            }
            ui.menu_button(BOOKMARK, |ui| {
                if ui
                    .button((BOOKMARK, if percent { "0.5%" } else { "0.005" }))
                    .clicked()
                {
                    self.auto.0 = 0.005;
                    self.is_auto = true;
                }
                if ui
                    .button((BOOKMARK, if percent { "1.0%" } else { "0.01" }))
                    .clicked()
                {
                    self.auto.0 = 0.01;
                    self.is_auto = true;
                };
            });
        });
    }

    /// Threshold sort
    fn sort(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Sort"))
                .on_hover_localized("Threshold_Sort.hover");
            ui.checkbox(&mut self.sort, ());
        });
    }

    /// Threshold filter
    fn filter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Threshold_Filter"))
                .on_hover_localized("Threshold_Filter.hover");
            ui.checkbox(&mut self.filter, ());
        });
    }
}
