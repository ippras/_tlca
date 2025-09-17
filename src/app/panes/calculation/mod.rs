use self::{
    settings::Settings,
    state::{State, Windows},
    statistics::Statistics,
    table::TableView,
};
use crate::{
    app::{
        HashedMetaDataFrame,
        computers::{CalculationComputed, CalculationKey, StatisticsComputed, StatisticsKey},
    },
    utils::{Hashed, save},
};
use anyhow::Result;
use egui::{
    CursorIcon, Id, Label, Pos2, Response, RichText, TextWrapMode, Ui, Widget, Window, util::hash,
};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, NOTE_PENCIL, PENCIL, SIGMA,
    SLIDERS_HORIZONTAL, TAG,
};
use metadata::egui::MetadataWidget;
use polars::prelude::*;
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use tracing::instrument;

const ID_SOURCE: &str = "Calculation";

/// Calculation pane
#[derive(Deserialize, Serialize)]
pub struct Pane {
    pub frames: Vec<HashedMetaDataFrame>,
    pub settings: Settings,
    id: Id,
    state: State,
}

impl Pane {
    pub fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        let id = Id::new(ID_SOURCE).with(&frames);
        Self {
            frames,
            settings: Settings::new(),
            state: State::new(),
            id,
        }
    }

    pub fn id(mut self, id_salt: impl Hash) -> Self {
        self.id = Id::new(ID_SOURCE).with(id_salt);
        self
    }

    pub const fn icon() -> &'static str {
        NOTE_PENCIL
    }

    pub fn title(&self) -> String {
        format_list_truncated!(self.frames.iter().map(|frame| frame.meta.format(".")), 2)
    }

    pub fn top(&mut self, ui: &mut Ui) -> Response {
        let mut windows = Windows::load(ui.ctx(), self.id);
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
        ui.add_enabled(self.frames.len() == 1, |ui: &mut Ui| {
            ui.toggle_value(&mut self.settings.editable, RichText::new(PENCIL).heading())
                .on_hover_text("edit")
        });
        ui.separator();
        // Settings
        ui.toggle_value(&mut windows.open_settings, RichText::new(GEAR).heading())
            .on_hover_text("settings");
        // Statistics
        ui.toggle_value(&mut windows.open_statistics, RichText::new(SIGMA).heading())
            .on_hover_ui(|ui| {
                ui.label("Statistics");
            });
        ui.separator();
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
        ui.separator();
        windows.store(ui.ctx(), self.id);
        response
    }

    pub fn central(&mut self, ui: &mut Ui) {
        self.windows(ui);
        if self.settings.editable {
            self.meta(ui);
        }
        self.data(ui);
    }

    fn meta(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        ui.collapsing(RichText::new(format!("{TAG} Metadata")).heading(), |ui| {
            MetadataWidget::new(&mut self.frames[0].meta)
                .with_writable(true)
                .show(ui);
        });
    }

    fn data(&mut self, ui: &mut Ui) {
        let _ = TableView::new(&mut self.frames, &self.settings, &mut self.state).show(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        hash(&self.frames)
    }

    fn save(&mut self) -> Result<()> {
        let frame = &mut self.frames[0];
        let name = format!("{}.tlca.ipc", frame.meta.format(".")).replace(" ", "_");
        // save(&name, &mut frame.value)?;
        Ok(())
    }
}

impl Pane {
    pub fn windows(&mut self, ui: &mut Ui) {
        let windows = &mut Windows::load(ui.ctx(), self.id);
        self.settings(ui, windows);
        self.statistics(ui, windows);
        windows.store(ui.ctx(), self.id);
    }

    fn settings(&mut self, ui: &mut Ui, windows: &mut Windows) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
            .default_pos(ui.next_widget_position())
            .open(&mut windows.open_settings)
            .show(ui.ctx(), |ui| {
                self.settings.show(ui);
            });
    }

    fn statistics(&mut self, ui: &mut Ui, windows: &mut Windows) {
        Window::new(format!("{SIGMA} Statistics"))
            .id(ui.auto_id_with(ID_SOURCE).with("Statistics"))
            .default_pos(ui.next_widget_position())
            .open(&mut windows.open_statistics)
            .show(ui.ctx(), |ui| self.statistics_content(ui));
    }

    // fn statistics(&mut self, ui: &mut Ui) {
    //     let target = ui.memory_mut(|memory| {
    //         memory
    //             .caches
    //             .cache::<CalculationComputed>()
    //             .get(CalculationKey {
    //                 frames: &self.frames,
    //                 parameters: &self.settings.parameters,
    //             })
    //     });
    //     let statistics = ui.memory_mut(|memory| {
    //         memory
    //             .caches
    //             .cache::<StatisticsComputed>()
    //             .get(StatisticsKey {
    //                 frame: &target,
    //                 settings: &self.settings,
    //             })
    //     });
    //     let _ = Statistics::new(&statistics).show(ui);
    // }
    #[instrument(skip_all, err)]
    fn statistics_content(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    frames: &self.frames,
                    parameters: &self.settings.parameters,
                })
        });
        let statistics = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<StatisticsComputed>()
                .get(StatisticsKey {
                    frame: &target,
                    settings: &self.settings,
                })
        });
        let _ = Statistics::new(&statistics).show(ui);
        // let data_frame = ui.memory_mut(|memory| {
        //     memory
        //         .caches
        //         .cache::<StatisticsComputed>()
        //         .get(StatisticsKey {
        //             // data_frame: &self.target,
        //             // from: self.parameters.from,
        //             // ddof: self.parameters.ddof,
        //         })
        // });
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
            settings: Default::default(),
            state: Default::default(),
            id: Id::new(ID_SOURCE),
        }
    }
}

pub mod settings;
pub mod statistics;

mod state;
mod table;
