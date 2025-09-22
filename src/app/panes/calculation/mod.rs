use self::{
    metrics::Metrics,
    state::{Settings, State},
    table::TableView,
};
use crate::{
    app::{
        HashedMetaDataFrame,
        computers::{CalculationComputed, CalculationKey, MetricsComputed, MetricsKey},
    },
    utils::save,
};
use anyhow::Result;
use egui::{
    Button, CursorIcon, Label, Response, RichText, TextWrapMode, Ui, Widget, Window, util::hash,
};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, NOTE_PENCIL, SIGMA, SLIDERS_HORIZONTAL,
    TAG,
};
use metadata::egui::MetadataWidget;
use polars::prelude::*;
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const ID_SOURCE: &str = "Calculation";

/// Calculation pane
#[derive(Deserialize, Serialize)]
pub struct Pane {
    pub frames: Vec<HashedMetaDataFrame>,
}

impl Pane {
    pub fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self { frames }
    }

    pub const fn icon() -> &'static str {
        NOTE_PENCIL
    }

    pub fn title(&self) -> String {
        format_list_truncated!(self.frames.iter().map(|frame| frame.meta.format(".")), 2)
    }

    pub fn top(&mut self, ui: &mut Ui, state: &mut State) -> Response {
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
            state.reset_table_state = true;
        }
        // Resize
        ui.toggle_value(
            &mut state.settings.resizable,
            RichText::new(ARROWS_HORIZONTAL).heading(),
        )
        .on_hover_text("ResizeTableColumns");
        // Edit metadata
        ui.add_enabled(self.frames.len() == 1, |ui: &mut Ui| {
            ui.toggle_value(&mut state.settings.editable, RichText::new(TAG).heading())
                .on_hover_text("EditMetadata")
        });
        ui.separator();
        // Settings
        ui.toggle_value(
            &mut state.windows.open_settings,
            RichText::new(GEAR).heading(),
        )
        .on_hover_text("ShowSettings");
        ui.separator();
        // Metrics
        ui.toggle_value(
            &mut state.windows.open_metrics,
            RichText::new(SIGMA).heading(),
        )
        .on_hover_ui(|ui| {
            ui.label("ShowMetrics");
        });
        ui.separator();
        // Save

        if ui
            .add_enabled(
                self.frames.len() == 1,
                Button::new(RichText::new(FLOPPY_DISK).heading()),
            )
            .on_hover_ui(|ui| {
                ui.label("Save");
            })
            .on_hover_text(format!("{}.tlca.parquet", self.frames[0].meta.format(".")))
            .clicked()
        {
            let _ = self.save();
        }
        ui.separator();
        response
    }

    pub fn central(&mut self, ui: &mut Ui, state: &mut State) {
        self.windows(ui, state);
        if state.settings.editable {
            self.meta(ui);
        }
        self.data(ui, state);
    }

    fn meta(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        ui.collapsing(RichText::new(format!("{TAG} Metadata")).heading(), |ui| {
            MetadataWidget::new(&mut self.frames[0].meta)
                .with_writable(true)
                .show(ui);
        });
    }

    fn data(&mut self, ui: &mut Ui, state: &mut State) {
        let _ = TableView::new(&mut self.frames, state).show(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        hash(&self.frames)
    }

    fn save(&mut self) -> Result<()> {
        let frame = &mut self.frames[0];
        let name = format!("{}.tlca.parquet", frame.meta.format(".")).replace(" ", "_");
        save(&name, frame)?;
        Ok(())
    }
}

impl Pane {
    pub fn windows(&mut self, ui: &mut Ui, state: &mut State) {
        self.settings(ui, state);
        self.metrics(ui, state);
    }

    fn settings(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_settings)
            .show(ui.ctx(), |ui| {
                state.settings.show(ui);
            });
    }

    fn metrics(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} Metrics"))
            .id(ui.auto_id_with(ID_SOURCE).with("Metrics"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_metrics)
            .show(ui.ctx(), |ui| self.metrics_content(ui, &state.settings));
    }

    #[instrument(skip_all, err)]
    fn metrics_content(&mut self, ui: &mut Ui, settings: &Settings) -> PolarsResult<()> {
        let frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    frames: &self.frames,
                    parameters: &settings.parameters,
                })
        });
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<MetricsComputed>().get(MetricsKey {
                frame: &frame,
                parameters: &settings.parameters,
            })
        });
        let _ = Metrics::new(&data_frame, settings).show(ui);
        // let settings = Settings::load(ui.ctx());
        // IndicesWidget::new(&data_frame)
        //     .precision(settings.precision)
        //     .show(ui)
        //     .inner
        Ok(())
    }
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            frames: Default::default(),
        }
    }
}

pub mod state;

mod metrics;
mod table;
