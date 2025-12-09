use self::{factors::Factors, indices::Indices, metrics::Metrics, table::TableView};
use super::{Behavior, MARGIN};
use crate::{
    app::{
        computers::fatty_acids::{
            Computed as FattyAcidsComputed, Key as FattyAcidsKey,
            factors::{Computed as FactorsComputed, Key as FactorsKey},
            indices::{Computed as IndicesComputed, Key as IndicesKey},
            metrics::{Computed as MetricsComputed, Key as MetricsKey},
            table::{Computed as TableComputed, Key as TableKey},
        },
        states::fatty_acids::{ID_SOURCE, Settings, State},
    },
    export::ron,
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use anyhow::Result;
use egui::{
    CentralPanel, CursorIcon, Frame, Id, Label, MenuBar, Response, RichText, ScrollArea, TextStyle,
    TextWrapMode, TopBottomPanel, Ui, Widget, Window, util::hash,
};
use egui_l20n::UiExt as _;
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, DROP, FLOPPY_DISK, GEAR, SIGMA, SLIDERS_HORIZONTAL, TAG, X,
};
use egui_tiles::{TileId, UiResponse};
use metadata::{egui::MetadataWidget, polars::MetaDataFrame};
use polars::prelude::*;
use polars_utils::{format_list, format_list_truncated};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, from_fn};
use tracing::instrument;

/// Fatty acids pane
#[derive(Default, Deserialize, Serialize)]
pub struct Pane {
    id: Option<Id>,
    frames: Vec<HashedMetaDataFrame>,
    calculated: HashedDataFrame,
}

impl Pane {
    pub(super) fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self {
            id: None,
            frames,
            calculated: HashedDataFrame::EMPTY,
        }
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
        _ = self.init(ui, &mut state);
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
                self.windows(ui, &mut state);
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

    fn init(&mut self, ui: &mut Ui, state: &mut State) {
        self.calculated = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<FattyAcidsComputed>()
                .get(FattyAcidsKey::new(&self.frames, &state.settings))
        });
    }

    fn top(&mut self, ui: &mut Ui, state: &mut State) -> Response {
        let mut response = ui.heading(DROP).on_hover_text("FattyAcids");
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(format!("{}/{:x}", self.id(), self.calculated.hash))
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
        self.sum_button(ui, state);
        ui.separator();
        self.save_button(ui);
        ui.separator();
        response
    }

    // Sum button
    fn sum_button(&self, ui: &mut Ui, state: &mut State) {
        ui.menu_button(RichText::new(SIGMA).heading(), |ui| {
            // Factors
            ui.toggle_value(
                &mut state.windows.open_factors,
                (
                    RichText::new(SIGMA).heading(),
                    RichText::new(ui.localize("Factors")).heading(),
                ),
            )
            .on_hover_ui(|ui| {
                ui.label(ui.localize("Factors"));
            });
            // Indices
            ui.toggle_value(
                &mut state.windows.open_indices,
                (
                    RichText::new(SIGMA).heading(),
                    RichText::new(ui.localize("Indices")).heading(),
                ),
            )
            .on_hover_ui(|ui| {
                ui.label(ui.localize("Indices"));
            });
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
        });
    }

    /// Save button
    fn save_button(&self, ui: &mut Ui) {
        ui.menu_button(RichText::new(FLOPPY_DISK).heading(), |ui| {
            let title = self.title();
            if ui
                .button("RON")
                .on_hover_ui(|ui| {
                    ui.label(ui.localize("Save"));
                })
                .on_hover_ui(|ui| {
                    ui.label(&format!("{title}.fa.utca.ron"));
                })
                .clicked()
            {
                _ = self.save_ron(&title);
            }
            // if ui
            //     .button("PARQUET")
            //     .on_hover_ui(|ui| {
            //         ui.label(ui.localize("Save"));
            //     })
            //     .on_hover_ui(|ui| {
            //         ui.label(&format!("{title}.fa.utca.parquet"));
            //     })
            //     .clicked()
            // {
            //     _ = self.save_parquet(&title);
            // }
            // _ = self.save();
        });
    }

    #[instrument(skip_all, err)]
    fn save_ron(&self, title: &str) -> Result<()> {
        let frame = &self.frames[0];
        let frame = MetaDataFrame::new(&frame.meta, &frame.data.data_frame);
        ron::save(&frame, &format!("{title}.fa.utca.ron"))?;
        Ok(())
    }

    fn central(&mut self, ui: &mut Ui, state: &mut State) {
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
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TableComputed>()
                .get(TableKey::new(&self.calculated, &state.settings))
        });
        _ = TableView::new(&data_frame, state).show(ui);
    }
}

impl Pane {
    fn windows(&mut self, ui: &mut Ui, state: &mut State) {
        self.settings(ui, state);
        self.factors(ui, state);
        self.indices(ui, state);
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

    fn factors(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} Factors"))
            .id(ui.auto_id_with(ID_SOURCE).with("Factors"))
            .open(&mut state.windows.open_factors)
            .show(ui.ctx(), |ui| self.factors_content(ui, &state.settings));
    }

    #[instrument(skip_all, err)]
    fn factors_content(&mut self, ui: &mut Ui, settings: &Settings) -> PolarsResult<()> {
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<FactorsComputed>()
                .get(FactorsKey::new(&self.calculated, settings))
        });
        Factors::new(&data_frame, settings).show(ui)
    }

    fn indices(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} Indices"))
            .id(ui.auto_id_with(ID_SOURCE).with("Indices"))
            .open(&mut state.windows.open_indices)
            .show(ui.ctx(), |ui| self.indices_content(ui, &state.settings));
    }

    #[instrument(skip_all, err)]
    fn indices_content(&mut self, ui: &mut Ui, settings: &Settings) -> PolarsResult<()> {
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<IndicesComputed>()
                .get(IndicesKey::new(&self.calculated, settings))
        });
        Indices::new(&data_frame, settings).show(ui)
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
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<MetricsComputed>()
                .get(MetricsKey::new(&self.calculated, settings))
        });
        _ = Metrics::new(&data_frame, settings).show(ui);
        Ok(())
    }
}

mod factors;
mod indices;
mod metrics;
mod table;
