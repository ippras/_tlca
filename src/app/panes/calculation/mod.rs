use std::f64::EPSILON;

use self::{settings::Settings, state::State, table::TableView};
use crate::{
    app::computers::{CalculationComputed, CalculationKey},
    utils::{AnyValueExt as _, Hashed, save},
};
use anyhow::Result;
use egui::{
    CursorIcon, Grid, Label, Response, RichText, TextStyle, TextWrapMode, Ui, Widget, Window,
    util::hash,
};
use egui_extras::{Size, StripBuilder};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, NOTE_PENCIL, PENCIL, TAG,
};
use metadata::{MetaDataFrame, egui::MetadataWidget};
use polars::{
    error::{PolarsError, PolarsResult},
    lazy::dsl::{max_horizontal, min_horizontal},
    prelude::*,
};
use polars_ext::expr::ExprExt;
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const ID_SOURCE: &str = "Calculation";

/// Calculation pane
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Pane {
    pub(crate) frames: Vec<Hashed<MetaDataFrame>>,
    pub(crate) settings: Settings,
    state: State,
}

impl Pane {
    pub(crate) fn new(frames: Vec<Hashed<MetaDataFrame>>) -> Self {
        Self {
            frames,
            settings: Settings::new(),
            state: State::new(),
        }
    }

    pub(crate) const fn icon() -> &'static str {
        NOTE_PENCIL
    }

    pub(crate) fn title(&self) -> String {
        format_list_truncated!(self.frames.iter().map(|frame| frame.meta.format(".")), 2)
    }

    pub(crate) fn top(&mut self, ui: &mut Ui) -> Response {
        let mut response = ui.heading(Self::icon()).on_hover_text("configuration");
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(format!("{:x}", self.hash()))
            .on_hover_ui(|ui| {
                Label::new(format_list!(
                    self.frames.iter().map(|frame| frame.meta.format("."))
                ))
                .wrap_mode(TextWrapMode::Extend)
                .ui(ui);
            })
            .on_hover_ui(|ui| {
                if let Some(frame) = self.frames.first() {
                    MetadataWidget::new(&frame.meta).show(ui);
                }
            })
            .on_hover_cursor(CursorIcon::Grab);
        ui.separator();
        // Reset
        if ui
            .button(RichText::new(ARROWS_CLOCKWISE).heading())
            .clicked()
        {
            self.state.reset_table_state = true;
        }
        // Resize
        ui.toggle_value(
            &mut self.settings.resizable,
            RichText::new(ARROWS_HORIZONTAL).heading(),
        )
        .on_hover_text("resize");
        // Edit
        ui.toggle_value(&mut self.settings.editable, RichText::new(PENCIL).heading())
            .on_hover_text("edit");
        ui.separator();
        // Settings
        ui.toggle_value(
            &mut self.state.open_settings_window,
            RichText::new(GEAR).heading(),
        )
        .on_hover_text("settings");
        ui.separator();
        // Save
        if ui
            .button(RichText::new(FLOPPY_DISK).heading())
            .on_hover_ui(|ui| {
                ui.label("save");
            })
            .on_hover_text(format!("{}.utca.ipc", self.frames[0].meta.format(".")))
            .clicked()
        {
            let _ = self.save();
        }
        response
    }

    pub(crate) fn central(&mut self, ui: &mut Ui) {
        self.windows(ui);
        if self.settings.editable {
            self.meta(ui);
        }
        self.data(ui);
    }

    #[instrument(skip(self, ui), err)]
    pub(crate) fn bottom(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    frames: &self.frames,
                    settings: &self.settings,
                })
        });
        let mut data_frame = target
            .clone()
            .lazy()
            .select([
                nth(1).fill_null(0).alias("Source"),
                nth(2).fill_null(0).alias("Target"),
            ])
            .select([
                (col("Source") - col("Target"))
                    .pow(2)
                    .sum()
                    .sqrt()
                    .alias("EuclideanDistance"),
                (col("Source") - col("Target"))
                    .abs()
                    .sum()
                    .alias("ManhattanDistance"),
                (lit(1)
                    - (col("Source") * col("Target")).sum()
                        / (col("Source").pow(2).sum().sqrt() * col("Target").pow(2).sum().sqrt()))
                .alias("CosineDistance"),
                ((col("Source") - col("Target")).abs().sum()
                    / (col("Source") + col("Target")).sum())
                .alias("BrayCurtisDissimilarity"),
                (lit(1)
                    - min_horizontal([col("Source"), col("Target")])?.sum()
                        / max_horizontal([col("Source"), col("Target")])?.sum())
                .alias("RuzickaDistance"),
            ])
            .collect()?;
        let m = || (col("Source") + col("Target")) / lit(2);
        let kl = |name: &str| (col(name) * (col(name) / m()).log1p()).sum();
        let lazy_frame = target
            .lazy()
            .select([
                (nth(1) + lit(EPSILON)).normalize().alias("Source"),
                (nth(2) + lit(EPSILON)).normalize().alias("Target"),
            ])
            .select([m().sum()
                .alias("JensenShannonDivergence")]);
            // .select([(lit(0.5) * kl("Source") + lit(0.5) * kl("Target"))
            //     .alias("JensenShannonDivergence")]);
        data_frame = data_frame.hstack(lazy_frame.collect()?.get_columns())?;
        // println!("data_frame: {data_frame}");
        let euclidean_distance = data_frame["EuclideanDistance"].get(0)?.display();
        let manhattan_distance = data_frame["ManhattanDistance"].get(0)?.display();
        let cosine_distance = data_frame["CosineDistance"].get(0)?.display();
        let bray_curtis_dissimilarity = data_frame["BrayCurtisDissimilarity"].get(0)?.display();
        let ruzicka_distance = data_frame["RuzickaDistance"].get(0)?.display();
        let jensen_shannon_divergence = data_frame["JensenShannonDivergence"].get(0)?.display();
        ui.heading("Метрики, основанные на геометрическом расстоянии");
        ui.small("Чувствительны к абсолютным значениям");
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.label("Euclidean distance")
                .on_hover_text("Евклидово расстояние");
            ui.label(euclidean_distance);
            ui.end_row();
            ui.label("Manhattan distance")
                .on_hover_text("Манхэттенское расстояние");
            ui.label(manhattan_distance);
            ui.end_row();
        });
        ui.heading("Метрики, основанные на схожести формы/профиля");
        ui.small("Менее чувствительны к абсолютным значениям, больше к относительным пропорциям");
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.label("Cosine distance")
                .on_hover_text("Косинусное расстояние")
                .on_hover_text("Расстояние 0 означает идеальное совпадение профилей. Расстояние 1 означает максимальную непохожесть (ортогональность для неотрицательных векторов).");
            ui.label(cosine_distance);
            ui.end_row();
        });
        ui.heading("Метрики, учитывающие наличие/отсутствие и величины");
        ui.small("Часто используются в экологии и для сравнения распределений");
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.label("Bray-Curtis dissimilarity")
                .on_hover_text("Расстояние Брея-Кёртиса")
                .on_hover_text("Варьируется от 0 (полное совпадение) до 1 (полное различие).");
            ui.label(bray_curtis_dissimilarity);
            ui.end_row();
            ui.label("Ruzicka distance")
                .on_hover_text("Ruzicka distance or weighted Jaccard distance")
                .on_hover_text("Расстояние Ружички")
                .on_hover_text("Варьируется от 0 (полное совпадение) до 1 (полное различие).");
            ui.label(ruzicka_distance);
            ui.end_row();
        });
        ui.heading("Информационно-теоретические метрики");
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.label("Jensen-Shannon divergence")
                .on_hover_text("Дивергенция Дженсена-Шеннона").on_hover_text("Варьируется от 0 (одинаковые распределения) до log(2) (для натурального логарифма) или 1 (для логарифма по основанию 2).");
            ui.label(jensen_shannon_divergence);
            ui.end_row();
        });
        Ok(())
    }

    fn meta(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        ui.collapsing(RichText::new(format!("{TAG} Metadata")).heading(), |ui| {
            MetadataWidget::new(&mut self.frames[0].value.meta)
                .with_writable(true)
                .show(ui);
        });
    }

    fn data(&mut self, ui: &mut Ui) {
        TableView::new(&mut self.frames, &self.settings, &mut self.state).show(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        hash(&self.frames)
    }

    pub(crate) fn windows(&mut self, ui: &mut Ui) {
        Window::new(format!("{GEAR} Settings"))
            .id(ui.auto_id_with(ID_SOURCE))
            .open(&mut self.state.open_settings_window)
            .show(ui.ctx(), |ui| self.settings.show(ui));
    }

    fn save(&mut self) -> Result<()> {
        let frame = &mut self.frames[0];
        let name = format!("{}.tlca.ipc", frame.meta.format(".")).replace(" ", "_");
        save(&name, &mut frame.value)?;
        Ok(())
    }
}

pub(crate) mod settings;

mod state;
mod table;
