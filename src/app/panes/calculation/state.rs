use super::ID_SOURCE;
use crate::app::{
    MAX_PRECISION,
    parameters::{Filter, Metric, Parameters, Sort, composition::COMPOSITIONS},
};
use egui::{ComboBox, Context, Grid, Id, Key, Slider, Ui, Widget};
use egui_ext::Markdown;
use serde::{Deserialize, Serialize};

const METRICS: [Metric; 11] = [
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
    //
    Metric::PearsonCorrelation,
    Metric::SpearmanRankCorrelation,
];

const SEPARATORS: [usize; 3] = [3, 6, 9];

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

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|data| {
            data.insert_persisted(id, self);
        });
    }
}

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
    pub rank: bool,

    pub parameters: Parameters,
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

            rank: true,

            parameters: Parameters::new(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ui: &mut Ui) {
        let id_salt = Id::new(ID_SOURCE).with("Settings");
        Grid::new(id_salt).show(ui, |ui| {
            // Precision
            ui.label("Precision");
            Slider::new(&mut self.precision, 0..=MAX_PRECISION).ui(ui);
            ui.end_row();

            // Percent
            let mut response = ui.label("Percent");
            response |= ui.checkbox(&mut self.percent, "");
            response.on_hover_ui(|ui| {
                ui.label("Percent.hover");
            });
            ui.end_row();

            // Truncate
            let mut response = ui.label("Truncate");
            response |= ui.checkbox(&mut self.truncate, "");
            response.on_hover_ui(|ui| {
                ui.label("Truncate.hover");
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

            ui.separator();
            ui.label("Metrics");
            ui.end_row();

            // Metric
            ui.label("Metric");
            ComboBox::from_id_salt(ui.auto_id_with(id_salt))
                .selected_text(self.parameters.metric.text())
                .show_ui(ui, |ui| {
                    for (index, metric) in METRICS.into_iter().enumerate() {
                        if SEPARATORS.contains(&index) {
                            ui.separator();
                        }
                        ui.selectable_value(&mut self.parameters.metric, metric, metric.text())
                            .on_hover_text(metric.hover_text());
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.markdown(self.parameters.metric.markdown());
                })
                .on_hover_text(self.parameters.metric.hover_text());
            ui.end_row();

            // Rank
            let mut response = ui.label("Rank");
            response |= ui.checkbox(&mut self.rank, "");
            response.on_hover_ui(|ui| {
                ui.label("Rank.hover");
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

/// Calculation windows
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Windows {
    pub open_metrics: bool,
    pub open_settings: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            open_metrics: false,
            open_settings: false,
        }
    }
}
