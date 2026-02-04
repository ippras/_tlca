use self::{metrics::Metrics, moments::Moments, table::TableView};
use super::{Behavior, MARGIN};
use crate::{
    app::{
        computers::triacylglycerols::{
            Computed as TriacylglycerolsComputed, Key as TriacylglycerolsKey,
            metrics::{Computed as MetricsComputed, Key as MetricsKey},
            moments::{Computed as MomentsComputed, Key as MomentsKey},
        },
        states::triacylglycerols::{ID_SOURCE, State, settings::Settings},
        widgets::buttons::{EditButton, MetadataButton, ResetButton, ResizeButton, SettingsButton},
    },
    export,
    utils::HashedMetaDataFrame,
};
use anyhow::Result;
use egui::{
    Button, CentralPanel, CursorIcon, Frame, Id, IntoAtoms, Label, MenuBar, Response, RichText,
    ScrollArea, TextStyle, TextWrapMode, TopBottomPanel, Ui, Widget, Window, util::hash,
};
use egui_l20n::prelude::*;
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, DROP, FLOPPY_DISK, SIGMA, SLIDERS_HORIZONTAL, TAG, X,
};
use egui_tiles::{TileId, UiResponse};
use metadata::egui::MetadataWidget;
use polars::prelude::*;
use polars_utils::format_list_truncated;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, from_fn};
use tracing::instrument;

/// Triacylglycerols pane
#[derive(Default, Deserialize, Serialize)]
pub struct Pane {
    id: Option<Id>,
    frames: Vec<HashedMetaDataFrame>,
}

impl Pane {
    pub(super) fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self { id: None, frames }
    }

    pub(super) fn title(&self) -> String {
        format_list_truncated!(self.frames.iter().map(|frame| frame.meta.format(".")), 2)
    }

    fn id(&self) -> impl Display {
        from_fn(|f| {
            if let Some(id) = self.id {
                write!(f, "{id:?}-")?;
            }
            write!(f, "{}", hash(&self.frames))
        })
    }
}

impl Pane {
    pub(super) fn ui(
        &mut self,
        ui: &mut Ui,
        behavior: &mut Behavior,
        tile_id: TileId,
    ) -> UiResponse {
        let id = *self.id.get_or_insert_with(|| ui.next_auto_id());
        let mut state = State::load(ui.ctx(), id);
        let response = TopBottomPanel::top(ui.auto_id_with("Pane"))
            .show_inside(ui, |ui| {
                MenuBar::new()
                    .ui(ui, |ui| {
                        ScrollArea::horizontal()
                            .show(ui, |ui| {
                                ui.set_height(
                                    ui.text_style_height(&TextStyle::Heading) + 4.0 * MARGIN.y,
                                );
                                ui.visuals_mut().button_frame = false;
                                if ui.button(RichText::new(X).heading()).clicked() {
                                    behavior.close = Some(tile_id);
                                }
                                ui.separator();
                                self.top(ui, &mut state)
                            })
                            .inner
                    })
                    .inner
            })
            .inner;
        CentralPanel::default()
            .frame(Frame::central_panel(ui.style()))
            .show_inside(ui, |ui| {
                self.central(ui, &mut state);
                self.windows(ui, &mut state); // Обязательно после central!
            });
        if behavior.close == Some(tile_id) {
            state.remove(ui.ctx(), id);
        } else {
            state.store(ui.ctx(), id);
        }
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }

    fn top(&mut self, ui: &mut Ui, state: &mut State) -> Response {
        let mut response = ui
            .heading((DROP, DROP, DROP).into_atoms().text().unwrap_or_default())
            .on_hover_text("Triacylglycerols");
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(self.id().to_string())
            .on_hover_ui(|ui| {
                if let Some(frame) = self.frames.first() {
                    MetadataWidget::new(&frame.meta).show(ui);
                }
            })
            .on_hover_cursor(CursorIcon::Grab);
        ui.separator();
        ResetButton::new(&mut state.event.reset_table_state).ui(ui);
        ResizeButton::new(&mut state.settings.resizable).ui(ui);
        ui.add_enabled_ui(self.frames.len() == 1, |ui| {
            EditButton::new(&mut state.settings.edit).ui(ui);
        });
        ui.separator();
        MetadataButton::new(&mut state.windows.open_metadata).ui(ui);
        ui.separator();
        SettingsButton::new(&mut state.windows.open_settings).ui(ui);
        ui.separator();
        // Sigma
        ui.menu_button(RichText::new(SIGMA).heading(), |ui| {
            // Metrics
            ui.toggle_value(
                &mut state.windows.open_metrics,
                (
                    RichText::new(SIGMA).heading(),
                    RichText::new(ui.localize("Metric?PluralCategory=other")).heading(),
                ),
            )
            .on_hover_ui(|ui| {
                ui.label(ui.localize("Metric?PluralCategory=other"));
            });
            // Moments
            ui.toggle_value(
                &mut state.windows.open_moments,
                (
                    RichText::new(SIGMA).heading(),
                    RichText::new(ui.localize("Moments")).heading(),
                ),
            )
            .on_hover_ui(|ui| {
                ui.label(ui.localize("Moments"));
            });
        });
        ui.separator();
        // Save
        self.save_button(ui, state);
        ui.separator();
        response
    }

    // Save button
    fn save_button(&self, ui: &mut Ui, state: &State) {
        ui.add_enabled_ui(self.frames.len() == 1, |ui| {
            ui.menu_button(RichText::new(FLOPPY_DISK).heading(), |ui| {
                let name = self.frames[0].meta.format(".");
                if ui
                    .button((FLOPPY_DISK, "RON"))
                    .on_hover_localized("Save")
                    .on_hover_ui(|ui| {
                        ui.label(format!("{name}.tag.utca.ron"));
                    })
                    .clicked()
                {
                    _ = self.save_ron(&name, state);
                }
            });
        });
    }

    #[instrument(skip(self, state), err)]
    fn save_ron(&self, name: impl Debug + Display, state: &State) -> Result<()> {
        export::ron::save(&self.frames[0], &format!("{name}.utca.ron"))
    }

    fn central(&mut self, ui: &mut Ui, state: &mut State) {
        _ = TableView::new(&mut self.frames, state).show(ui);
    }
}

impl Pane {
    fn windows(&mut self, ui: &mut Ui, state: &mut State) {
        self.metadata_window(ui, state);
        self.metrics_window(ui, state);
        self.moments_window(ui, state);
        self.settings_window(ui, state);
    }

    fn metadata_window(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{TAG} Configuration metadata"))
            .id(ui.auto_id_with(ID_SOURCE).with("Metadata"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_metadata)
            .show(ui.ctx(), |ui| {
                MetadataWidget::new(&mut self.frames[0].meta)
                    .with_writable(state.settings.edit)
                    .show(ui);
            });
    }

    fn metrics_window(&mut self, ui: &mut Ui, state: &mut State) {
        if let Some(inner_response) = Window::new(format!("{SIGMA} Metrics"))
            .id(ui.auto_id_with(ID_SOURCE).with("Metrics"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_metrics)
            .show(ui.ctx(), |ui| self.metrics_content(ui, &state.settings))
        {
            inner_response.response.on_hover_ui(|ui| {
                ui.label(format!("{DROP}{DROP}{DROP} {}", self.title()));
            });
        }
    }

    #[instrument(skip_all, err)]
    fn metrics_content(&mut self, ui: &mut Ui, settings: &Settings) -> PolarsResult<()> {
        let frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TriacylglycerolsComputed>()
                .get(TriacylglycerolsKey::new(&self.frames, settings))
        });
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<MetricsComputed>()
                .get(MetricsKey::new(&frame, &settings))
        });
        _ = Metrics::new(&data_frame, settings).show(ui);
        Ok(())
    }

    fn moments_window(&mut self, ui: &mut Ui, state: &mut State) {
        if let Some(inner_response) = Window::new(format!("{SIGMA} Moments"))
            .id(ui.auto_id_with(ID_SOURCE).with("Moments"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_moments)
            .show(ui.ctx(), |ui| self.moments_content(ui, &state.settings))
        {
            inner_response.response.on_hover_ui(|ui| {
                ui.label(format!("{DROP}{DROP}{DROP} {}", self.title()));
            });
        }
    }

    #[instrument(skip_all, err)]
    fn moments_content(&mut self, ui: &mut Ui, settings: &Settings) -> PolarsResult<()> {
        let frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TriacylglycerolsComputed>()
                .get(TriacylglycerolsKey::new(&self.frames, settings))
        });
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<MomentsComputed>().get(MomentsKey {
                frame: &frame,
                bias: settings.bias,
            })
        });
        _ = Moments::new(&data_frame, settings).show(ui);
        Ok(())
    }

    fn settings_window(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
            .default_pos(ui.next_widget_position())
            .open(&mut state.windows.open_settings)
            .show(ui.ctx(), |ui| {
                state.settings.show(ui);
            });
    }
}

mod metrics;
mod moments;
mod table;
