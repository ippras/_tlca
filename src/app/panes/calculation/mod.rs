use self::{settings::Settings, state::State, statistics::Statistics, table::TableView};
use crate::{
    app::computers::{CalculationComputed, CalculationKey, StatisticsComputed, StatisticsKey},
    markdown::*,
    utils::{AnyValueExt as _, Hashed, LayoutJobExt as _, UiExt, save},
};
use anyhow::Result;
use egui::{
    CursorIcon, Grid, Label, Response, RichText, ScrollArea, TextStyle, TextWrapMode, Ui, Widget,
    Window, text::LayoutJob, util::hash,
};
use egui_extras::{Size, StripBuilder};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, INFO, NOTE_PENCIL, PENCIL, SIGMA,
    TABLE, TAG,
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
use settings::View;
use std::f64::{EPSILON, consts::E};
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
        // Statistics
        let text = match self.settings.view {
            View::Data => TABLE,
            View::Statistics => SIGMA,
        };
        ui.menu_button(RichText::new(text).heading(), |ui| {
            let mut response = ui
                .selectable_value(&mut self.settings.view, View::Data, View::Data.text())
                .on_hover_text(View::Data.hover_text());
            response |= ui
                .selectable_value(
                    &mut self.settings.view,
                    View::Statistics,
                    View::Statistics.text(),
                )
                .on_hover_text(View::Statistics.hover_text());
            if response.clicked() {
                ui.close_menu();
            }
        });
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
        match self.settings.view {
            View::Data => self.data(ui),
            View::Statistics => self.statistics(ui),
        }
    }

    pub(crate) fn statistics(&mut self, ui: &mut Ui) {
        let target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    frames: &self.frames,
                    settings: &self.settings,
                })
        });
        let statistics = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<StatisticsComputed>()
                .get(StatisticsKey {
                    frame: &Hashed {
                        value: target,
                        hash: hash((&self.frames, &self.settings)),
                    },
                    settings: &self.settings,
                })
        });
        let _ = Statistics::new(&statistics).show(ui);
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
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
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
pub(crate) mod statistics;

mod state;
mod table;
