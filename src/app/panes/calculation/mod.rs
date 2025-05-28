use self::{settings::Settings, state::State, table::TableView};
use crate::utils::Hashed;
use egui::{CursorIcon, Label, Response, RichText, TextWrapMode, Ui, Widget, Window, util::hash};
use egui_phosphor::regular::{ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, GEAR, NOTE_PENCIL, PENCIL};
use metadata::{MetaDataFrame, egui::MetadataWidget};
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};

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
        response
    }

    pub(crate) fn central(&mut self, ui: &mut Ui) {
        self.windows(ui);
        self.body(ui);
    }

    fn body(&mut self, ui: &mut Ui) {
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
}

pub(crate) mod settings;

mod state;
mod table;
