use std::f64::{EPSILON, consts::E};

use self::{settings::Settings, state::State, table::TableView};
use crate::{
    app::computers::{CalculationComputed, CalculationKey},
    utils::{AnyValueExt as _, Hashed, LayoutJobExt as _, save},
};
use anyhow::Result;
use egui::{
    CursorIcon, Grid, Label, Response, RichText, TextStyle, TextWrapMode, Ui, Widget, Window,
    text::LayoutJob, util::hash,
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

        const LEFT: &str = "Left";
        const RIGHT: &str = "Right";

        let mut data_frame = target
            .clone()
            .lazy()
            .select([
                nth(1).fill_null(0).alias(LEFT),
                nth(2).fill_null(0).alias(RIGHT),
            ])
            .select([
                (col(LEFT) - col(RIGHT))
                    .pow(2)
                    .sum()
                    .sqrt()
                    .alias("EuclideanDistance"),
                (col(LEFT) - col(RIGHT))
                    .abs()
                    .sum()
                    .alias("ManhattanDistance"),
                (lit(1)
                    - (col(LEFT) * col(RIGHT)).sum()
                        / (col(LEFT).pow(2).sum().sqrt() * col(RIGHT).pow(2).sum().sqrt()))
                .alias("CosineDistance"),
                ((col(LEFT) - col(RIGHT)).abs().sum() / (col(LEFT) + col(RIGHT)).sum())
                    .alias("BrayCurtisDissimilarity"),
                (lit(1)
                    - min_horizontal([col(LEFT), col(RIGHT)])?.sum()
                        / max_horizontal([col(LEFT), col(RIGHT)])?.sum())
                .alias("RuzickaDistance"),
            ])
            .collect()?;
        let m = || (col(LEFT) + col(RIGHT)) / lit(2);
        let kld = |left, rigth| (col(left) * (col(left) / m()).log(E)).fill_nan(0).sum();
        let jsd = || (lit(0.5) * (kld(LEFT) + kld(RIGHT)));
        let lazy_frame = target
            .lazy()
            .select([
                nth(1).fill_null(0).alias(LEFT),
                nth(2).fill_null(0).alias(RIGHT),
            ])
            .select([
                (col(LEFT) / col(LEFT).sum()),
                (col(RIGHT) / col(RIGHT).sum()),
            ])
            .select([
                as_struct(vec![
                    kld(LEFT).alias("LeftRight"),
                    kld(RIGHT).alias("RightLeft"),
                ])
                .alias("KullbackLeiblerDivergence"),
                as_struct(vec![
                    jsd().alias("Divergence"),
                    jsd().sqrt().alias("Distance"),
                ])
                .alias("JensenShannon"),
            ]);
        // unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", 256.to_string()) };
        // println!("target.slice: {}", target.slice(0, 12));
        // println!("->lazy_frame: {}", lazy_frame.clone().collect()?);
        data_frame = data_frame.hstack(lazy_frame.collect()?.get_columns())?;
        // println!("data_frame: {data_frame}");
        let euclidean_distance = data_frame["EuclideanDistance"].get(0)?.display();
        let manhattan_distance = data_frame["ManhattanDistance"].get(0)?.display();
        let cosine_distance = data_frame["CosineDistance"].get(0)?.display();
        let bray_curtis_dissimilarity = data_frame["BrayCurtisDissimilarity"].get(0)?.display();
        let ruzicka_distance = data_frame["RuzickaDistance"].get(0)?.display();
        let kullback_leibler_divergence = data_frame["KullbackLeiblerDivergence"].struct_()?;
        let kullback_leibler_divergence_left_right = kullback_leibler_divergence
            .field_by_name("Divergence")?
            .get(0)?
            .display();
        let jensen_shannon = data_frame["JensenShannon"].struct_()?;
        let jensen_shannon_divergence = jensen_shannon
            .field_by_name("Divergence")?
            .get(0)?
            .display();
        let jensen_shannon_distance = jensen_shannon.field_by_name("Distance")?.get(0)?.display();
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
            ui.label("Kullback-Leibler divergence");
            ui.end_row();
            ui.label(LayoutJob::subscripted_text(
                ui,
                "D_KL",
                Some(TextStyle::Heading),
                None,
            ));
            // ui.end_row();
            // ui.label("Divergence")
            //     .on_hover_text("Jensen-Shannon divergence")
            //     .on_hover_text("Дивергенция Дженсена-Шеннона").on_hover_text("Варьируется от 0 (одинаковые распределения) до log(2) (для натурального логарифма) или 1 (для логарифма по основанию 2).");
            // ui.label(jensen_shannon_divergence);
            // ui.end_row();
            ui.label("Jensen-Shannon distance")
                .on_hover_text("Jensen-Shannon distance")
                .on_hover_text("Расстояние Дженсена-Шеннона")
                .on_hover_text("");
            ui.label(jensen_shannon_distance).on_hover_ui(|ui| {
                Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    ui.label("Jensen-Shannon divergence");
                    ui.label(jensen_shannon_divergence);
                });
            });
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
