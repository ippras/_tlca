use self::{factors::Factors, indices::Indices, metrics::Metrics, view::table::TableView};
use super::{Behavior, MARGIN};
use crate::{
    app::{
        computers::fatty_acids::{
            factors::{Computed as FactorsComputed, Key as FactorsKey},
            join::{Computed as JoinComputed, Key as JoinKey},
            metrics::{Computed as MetricsComputed, Key as MetricsKey},
            select::{Computed as SelectComputed, Key as SelectKey},
            sum::sum::{Computed as SumComputed, Key as SumKey},
            view::table::{Computed as TableComputed, Key as TableKey},
        },
        panes::fatty_acids::sum::expressions::Expressions,
        states::fatty_acids::{ID_SOURCE, State, settings::Settings},
    },
    r#const::MAJOR,
    export::ron,
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use anyhow::Result;
use const_format::formatcp;
use egui::{
    CentralPanel, CursorIcon, Frame, Id, Label, MenuBar, Panel, Response, RichText, ScrollArea,
    TextStyle, TextWrapMode, Ui, Widget, Window, util::hash,
};
use egui_l10n::prelude::*;
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, DROP, FLOPPY_DISK, GEAR, SIGMA, SLIDERS_HORIZONTAL, TAG, X,
};
use egui_tiles::{TileId, UiResponse};
use fatty_acid_expressions::r#const::{
    BIODIESEL, EXPRESSION, METABOLIC, NUTRITIONAL, PREFIX as FAE, RATIO, SUM,
};
use metadata::{egui::MetadataWidget, polars::MetaDataFrame};
use polars::prelude::*;
use polars_ext::list::format_list_truncated;
use polars_utils::format_list;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, from_fn};
use tracing::instrument;
use widgets::buttons::{MetadataButton, ResetButton, ResizableButton, SettingsButton};

/// Fatty acids pane
#[derive(Default, Deserialize, Serialize)]
pub struct Pane {
    id: Option<Id>,
    frames: Vec<HashedMetaDataFrame>,
    select: HashedDataFrame,
}

impl Pane {
    pub(super) fn new(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self {
            id: None,
            frames,
            select: HashedDataFrame::EMPTY,
        }
    }

    pub(super) fn title(&self) -> String {
        format_list_truncated::<2>(self.frames.iter().map(|frame| frame.meta.format(".")))
            .to_string()
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
        let response = Panel::top(ui.auto_id_with("Pane"))
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

    #[instrument(skip_all, err)]
    fn init(&mut self, ui: &mut Ui, state: &mut State) -> PolarsResult<()> {
        self.select = ui.memory_mut(|memory| {
            let join = memory
                .caches
                .cache::<JoinComputed>()
                .get(JoinKey::new(&self.frames, &state.settings))
                .clone();
            memory
                .caches
                .cache::<SelectComputed>()
                .get(SelectKey::new(&join, &state.settings))
                .clone()
        });
        state.settings.major.manual = self.select[MAJOR].bool()?.into_no_null_iter().collect();
        Ok(())
    }

    fn top(&mut self, ui: &mut Ui, state: &mut State) -> Response {
        let mut response = ui.heading(DROP).on_hover_text("FattyAcids");
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(format!("{}/{:x}", self.id(), self.select.hash))
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
        ResetButton::builder()
            .selected(&mut state.settings.reset)
            .build()
            .ui(ui);
        ResizableButton::builder()
            .selected(&mut state.settings.resizable)
            .build()
            .ui(ui);
        // MetadataButton::builder()
        //     .selected(&mut state.windows.open_metadata)
        //     .build()
        //     .ui(ui);
        ui.separator();
        SettingsButton::builder()
            .selected(&mut state.windows.open_settings)
            .build()
            .ui(ui);
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

            // Fatty acid expressions
            ui.menu_button(
                (
                    RichText::new(SIGMA).heading(),
                    RichText::new(
                        ui.localize(formatcp!("{FAE}_{EXPRESSION}?PluralCategory=other")),
                    )
                    .heading(),
                ),
                |ui| {
                    // Sum
                    ui.toggle_value(
                        &mut state.windows.open_expressions_sum,
                        (
                            RichText::new(SIGMA).heading(),
                            RichText::new(ui.localize(formatcp!("{FAE}_{SUM}"))).heading(),
                        ),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(formatcp!("{FAE}_{SUM}")));
                    });
                    // Ratio
                    ui.menu_button(
                        (
                            RichText::new(SIGMA).heading(),
                            RichText::new(
                                ui.localize(formatcp!("{FAE}_{RATIO}?PluralCategory=other")),
                            )
                            .heading(),
                        ),
                        |ui| {
                            // Biodiesel
                            ui.toggle_value(
                                &mut state.windows.open_expressions_ratio_biodiesel,
                                (
                                    RichText::new(SIGMA).heading(),
                                    RichText::new(
                                        ui.localize(formatcp!("{FAE}_{RATIO}_{BIODIESEL}")),
                                    )
                                    .heading(),
                                ),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(ui.localize(formatcp!("{FAE}_{RATIO}_{BIODIESEL}")));
                            });
                            // Metabolic
                            ui.toggle_value(
                                &mut state.windows.open_expressions_ratio_metabolic,
                                (
                                    RichText::new(SIGMA).heading(),
                                    RichText::new(
                                        ui.localize(formatcp!("{FAE}_{RATIO}_{METABOLIC}")),
                                    )
                                    .heading(),
                                ),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(ui.localize(formatcp!("{FAE}_{RATIO}_{METABOLIC}")));
                            });
                            // Nutritional
                            ui.toggle_value(
                                &mut state.windows.open_expressions_ratio_nutritional,
                                (
                                    RichText::new(SIGMA).heading(),
                                    RichText::new(
                                        ui.localize(formatcp!("{FAE}_{RATIO}_{NUTRITIONAL}")),
                                    )
                                    .heading(),
                                ),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(ui.localize(formatcp!("{FAE}_{RATIO}_{NUTRITIONAL}")));
                            });
                        },
                    )
                    .response
                    .on_hover_ui(|ui| {
                        ui.label(
                            ui.localize(formatcp!("{FAE}_{RATIO}.hover?PluralCategory=other")),
                        );
                    });
                },
            )
            .response
            .on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{FAE}_{EXPRESSION}.hover?PluralCategory=other")));
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
                .get(TableKey::new(&self.select, &state.settings))
                .clone()
        });
        _ = TableView::new(&data_frame, &mut state.settings).show(ui);
    }
}

impl Pane {
    fn windows(&mut self, ui: &mut Ui, state: &mut State) {
        self.settings(ui, state);
        self.factors(ui, state);
        self.metrics(ui, state);
        self.expressions_sum(ui, state);
        self.expressions_ratio_biodiesel(ui, state);
        self.expressions_ratio_metabolic(ui, state);
        self.expressions_ratio_nutritional(ui, state);
    }

    fn settings(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
            .constrain_to(ui.clip_rect())
            .open(&mut state.windows.open_settings)
            .show(ui.ctx(), |ui| {
                state.settings.show(ui);
            });
    }

    fn expressions_ratio_biodiesel(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!(
            "{SIGMA} {RATIO} {}",
            ui.localize(formatcp!("{FAE}_{RATIO}_{BIODIESEL}"))
                .to_lowercase()
        ))
        .id(ui
            .auto_id_with(ID_SOURCE)
            .with(EXPRESSION)
            .with(RATIO)
            .with(BIODIESEL))
        .constrain_to(ui.clip_rect())
        .open(&mut state.windows.open_expressions_ratio_biodiesel)
        .show(ui.ctx(), |ui| {
            top(ui, &mut state.settings);
            let data_frame = ui.memory_mut(|memory| {
                memory
                    .caches
                    .cache::<SumComputed>()
                    .get(SumKey::new(&self.select, &state.settings))
                    .clone()
            });
            Expressions::new(&data_frame, &mut state.settings).show(ui);
        });
    }

    fn expressions_ratio_metabolic(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} {SUM} expressions"))
            .id(ui
                .auto_id_with(ID_SOURCE)
                .with(EXPRESSION)
                .with(RATIO)
                .with(METABOLIC))
            .constrain_to(ui.clip_rect())
            .open(&mut state.windows.open_expressions_ratio_metabolic)
            .show(ui.ctx(), |ui| {
                top(ui, &mut state.settings);
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<SumComputed>()
                        .get(SumKey::new(&self.select, &state.settings))
                        .clone()
                });
                Expressions::new(&data_frame, &mut state.settings).show(ui);
            });
    }

    fn expressions_ratio_nutritional(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} {SUM} expressions"))
            .id(ui
                .auto_id_with(ID_SOURCE)
                .with(EXPRESSION)
                .with(RATIO)
                .with(NUTRITIONAL))
            .constrain_to(ui.clip_rect())
            .open(&mut state.windows.open_expressions_ratio_nutritional)
            .show(ui.ctx(), |ui| {
                top(ui, &mut state.settings);
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<SumComputed>()
                        .get(SumKey::new(&self.select, &state.settings))
                        .clone()
                });
                Expressions::new(&data_frame, &mut state.settings).show(ui);
            });
    }

    fn expressions_sum(&mut self, ui: &mut Ui, state: &mut State) {
        Window::new(format!("{SIGMA} {SUM} expressions"))
            .id(ui.auto_id_with(ID_SOURCE).with(EXPRESSION).with(SUM))
            .constrain_to(ui.clip_rect())
            .open(&mut state.windows.open_expressions_sum)
            .show(ui.ctx(), |ui| {
                top(ui, &mut state.settings);
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<SumComputed>()
                        .get(SumKey::new(&self.select, &state.settings))
                        .clone()
                });
                Expressions::new(&data_frame, &mut state.settings).show(ui);
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
                .get(FactorsKey::new(&self.select, settings))
                .clone()
        });
        Factors::new(&data_frame, settings).show(ui)
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
                .get(MetricsKey::new(&self.select, settings))
                .clone()
        });
        _ = Metrics::new(&data_frame, settings).show(ui);
        Ok(())
    }
}

fn top(ui: &mut Ui, settings: &mut Settings) {
    Panel::top(ui.auto_id_with("Top")).show_inside(ui, |ui| {
        MenuBar::new()
            .ui(ui, |ui| {
                ScrollArea::horizontal()
                    .show(ui, |ui| {
                        ui.visuals_mut().button_frame = false;

                        ui.heading(ui.localize(settings.stereospecific_numbers.text()));
                        ui.separator();
                        ResetButton::builder()
                            .selected(&mut settings.reset)
                            .build()
                            .ui(ui);
                        ResizableButton::builder()
                            .selected(&mut settings.resizable)
                            .build()
                            .ui(ui);
                        ui.separator();
                        // ui.menu_button(RichText::new(FLOPPY_DISK).heading(), |ui| {
                        //     let title = self.title();
                        //     if ui
                        //         .button("RON")
                        //         .on_hover_ui(|ui| {
                        //             ui.label(ui.localize("Save"));
                        //         })
                        //         .on_hover_ui(|ui| {
                        //             ui.label(&format!("{title}.fa.utca.ron"));
                        //         })
                        //         .clicked()
                        //     {
                        //         _ = self.save_ron(&title);
                        //     }
                        // });
                    })
                    .inner
            })
            .inner;
    });
}

// /// Save button
// fn save_button(&self, ui: &mut Ui) {
//     ui.menu_button(RichText::new(FLOPPY_DISK).heading(), |ui| {
//         let title = self.title();
//         if ui
//             .button("RON")
//             .on_hover_ui(|ui| {
//                 ui.label(ui.localize("Save"));
//             })
//             .on_hover_ui(|ui| {
//                 ui.label(&format!("{title}.fa.utca.ron"));
//             })
//             .clicked()
//         {
//             _ = self.save_ron(&title);
//         }
//     })
// }

mod factors;
mod indices;
mod metrics;
mod sum;
mod view;
