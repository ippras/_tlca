use self::{settings::Settings, state::State, table::TableView};
use crate::utils::save;
use anyhow::Result;
use egui::{CursorIcon, Response, RichText, Ui, Window, util::hash};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, ERASER, FLOPPY_DISK, GEAR, NOTE_PENCIL, PENCIL, TAG,
};
use metadata::{MetaDataFrame, egui::MetadataWidget};
use serde::{Deserialize, Serialize};
use tracing::error;

const ID_SOURCE: &str = "Calculation";

/// Calculation pane
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Pane {
    pub(crate) frame: MetaDataFrame,
    pub(crate) settings: Settings,
    state: State,
}

impl Pane {
    pub(crate) fn new(frame: MetaDataFrame) -> Self {
        Self {
            frame,
            settings: Settings::new(),
            state: State::new(),
        }
    }

    pub(crate) const fn icon() -> &'static str {
        NOTE_PENCIL
    }

    pub(crate) fn title(&self) -> String {
        self.frame.meta.format(" ").to_string()
    }

    pub(crate) fn header(&mut self, ui: &mut Ui) -> Response {
        let mut response = ui.heading(Self::icon()).on_hover_text("configuration");
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(format!("{:x}", self.hash()))
            .on_hover_ui(|ui| {
                MetadataWidget::new(&mut self.frame.meta).show(ui);
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
        // Clear
        ui.add_enabled_ui(
            self.settings.editable && self.frame.data.height() > 0,
            |ui| {
                if ui
                    .button(RichText::new(ERASER).heading())
                    .on_hover_text("clear")
                    .clicked()
                {
                    self.frame.data = self.frame.data.clear();
                }
            },
        );
        ui.separator();
        // Settings
        ui.toggle_value(
            &mut self.state.open_settings_window,
            RichText::new(GEAR).heading(),
        )
        .on_hover_text("settings");
        ui.separator();
        if ui
            .button(RichText::new(FLOPPY_DISK).heading())
            .on_hover_text("save")
            .on_hover_text(&self.settings.label)
            .clicked()
        {
            if let Err(error) = self.save() {
                error!(%error);
            }
        }
        ui.separator();
        response
    }

    pub(crate) fn body(&mut self, ui: &mut Ui) {
        self.windows(ui);
        if self.settings.editable {
            self.body_content_meta(ui);
        }
        self.body_content_data(ui);
    }

    fn body_content_meta(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        ui.collapsing(RichText::new(format!("{TAG} Metadata")).heading(), |ui| {
            MetadataWidget::new(&mut self.frame.meta)
                .with_writable(true)
                .show(ui);
        });
    }

    fn body_content_data(&mut self, ui: &mut Ui) {
        TableView::new(&mut self.frame.data, &self.settings, &mut self.state).show(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        hash(&self.frame)
    }

    fn save(&mut self) -> Result<()> {
        let name = self.frame.meta.format(".").to_string().replace(" ", "_") + ".tlca.ipc";
        save(&name, &mut self.frame)?;
        Ok(())
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
