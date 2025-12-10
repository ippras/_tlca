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
    },
    utils::HashedMetaDataFrame,
};
use egui::{
    Button, CentralPanel, CursorIcon, Frame, Id, IntoAtoms, Label, MenuBar, Response, RichText,
    ScrollArea, TextStyle, TextWrapMode, TopBottomPanel, Ui, Widget, Window, util::hash,
};
use egui_l20n::prelude::*;
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, DROP, FLOPPY_DISK, GEAR, SIGMA, SLIDERS_HORIZONTAL, TAG, X,
};
use egui_tiles::{TileId, UiResponse};
use metadata::egui::MetadataWidget;
use polars::prelude::*;
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Triacylglycerols pane
#[derive(Default, Deserialize, Serialize)]
pub struct Pane {
    frames: Vec<HashedMetaDataFrame>,
}

impl Pane {
    pub(super) fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self { frames }
    }

    pub(super) fn title(&self) -> String {
        format_list_truncated!(self.frames.iter().map(|frame| frame.meta.format(".")), 2)
    }

    fn hash(&self) -> u64 {
        hash(&self.frames)
    }
}

impl Pane {
    pub(super) fn ui(
        &mut self,
        ui: &mut Ui,
        behavior: &mut Behavior,
        tile_id: TileId,
    ) -> UiResponse {
        let mut state = State::load(ui.ctx(), Id::new(tile_id));
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
            .frame(Frame::central_panel(&ui.style()))
            .show_inside(ui, |ui| {
                self.central(ui, &mut state);
            });
        if let Some(id) = behavior.close {
            state.remove(ui.ctx(), Id::new(id));
        } else {
            state.store(ui.ctx(), Id::new(tile_id));
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
        if ui
            .add_enabled(
                self.frames.len() == 1,
                Button::new(RichText::new(FLOPPY_DISK).heading()),
            )
            .on_hover_ui(|ui| {
                ui.label("Save");
            })
            .on_hover_text(format!(
                "{}.tag.utca.parquet",
                self.frames[0].meta.format(".")
            ))
            .clicked()
        {
            // _ = self.save();
        }
        ui.separator();
        response
    }

    // fn save(&mut self) -> Result<()> {
    //     let frame = &mut self.frames[0];
    //     let name = format!("{}.fa.parquet", frame.meta.format(".")).replace(" ", "_");
    //     save(&name, frame)?;
    //     Ok(())
    // }

    fn central(&mut self, ui: &mut Ui, state: &mut State) {
        self.windows(ui, state);
        if state.settings.editable {
            self.meta(ui);
            ui.separator();
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
        _ = TableView::new(&mut self.frames, state).show(ui);
    }
}

impl Pane {
    fn windows(&mut self, ui: &mut Ui, state: &mut State) {
        self.settings(ui, state);
        self.metrics(ui, state);
        self.moments(ui, state);
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

    fn moments(&mut self, ui: &mut Ui, state: &mut State) {
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
}

mod metrics;
mod moments;
mod table;
