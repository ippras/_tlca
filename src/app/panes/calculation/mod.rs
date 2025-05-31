use self::{settings::Settings, state::State, statistics::Statistics, table::TableView};
use crate::{
    app::computers::{CalculationComputed, CalculationKey, StatisticsComputed, StatisticsKey},
    markdown::*,
    utils::{AnyValueExt as _, Hashed, LayoutJobExt as _, UiExt, save},
};
use anyhow::Result;
use egui::{
    CursorIcon, Grid, Label, Response, RichText, TextStyle, TextWrapMode, Ui, Widget, Window,
    text::LayoutJob, util::hash,
};
use egui_extras::{Size, StripBuilder};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, INFO, NOTE_PENCIL, PENCIL, SIGMA, TAG,
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
        ui.toggle_value(
            &mut self.state.open_statistics_window,
            RichText::new(SIGMA).heading(),
        )
        .on_hover_text("statistics");
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

        // const LEFT: &str = "Left";
        // const RIGHT: &str = "Right";
        // let mut data_frame = target
        //     .clone()
        //     .slice(0, 12)
        //     .lazy()
        //     .select([
        //         nth(1).fill_null(0).alias(LEFT),
        //         nth(2).fill_null(0).alias(RIGHT),
        //     ])
        //     .select([
        //         (col(LEFT) - col(RIGHT))
        //             .pow(2)
        //             .sum()
        //             .sqrt()
        //             .alias("EuclideanDistance"),
        //         (col(LEFT) - col(RIGHT)).max().alias("ChebyshevDistance"),
        //         (col(LEFT) - col(RIGHT))
        //             .abs()
        //             .sum()
        //             .alias("ManhattanDistance"),
        //         (lit(1)
        //             - (col(LEFT) * col(RIGHT)).sum()
        //                 / (col(LEFT).pow(2).sum().sqrt() * col(RIGHT).pow(2).sum().sqrt()))
        //         .alias("CosineDistance"),
        //         ((col(LEFT) - col(RIGHT)).abs().sum() / (col(LEFT) + col(RIGHT)).sum())
        //             .alias("BrayCurtisDissimilarity"),
        //         (lit(1)
        //             - min_horizontal([col(LEFT), col(RIGHT)])?.sum()
        //                 / max_horizontal([col(LEFT), col(RIGHT)])?.sum())
        //         .alias("RuzickaDistance"),
        //         pearson_corr(col(LEFT), col(RIGHT)).alias("PearsonCorrelation"),
        //         spearman_rank_corr(col(LEFT), col(RIGHT), false).alias("SpearmanCorrelation"),
        //     ])
        //     .collect()?;
        // // Pearson and Spearman distances correlation
        // // 1-0.000351=0.999649
        // let m = || (col(LEFT) + col(RIGHT)) / lit(2);
        // let kld = |left: Expr, rigth| (left.clone() * (left / rigth).log(E)).fill_nan(0).sum();
        // let jsd = || (lit(0.5) * (kld(col(LEFT), m()) + kld(col(RIGHT), m())));
        // let lazy_frame = target
        //     .clone()
        //     .lazy()
        //     .select([
        //         nth(1).fill_null(0).alias(LEFT),
        //         nth(2).fill_null(0).alias(RIGHT),
        //     ])
        //     .select([
        //         (col(LEFT) / col(LEFT).sum()),
        //         (col(RIGHT) / col(RIGHT).sum()),
        //     ])
        //     .select([
        //         as_struct(vec![
        //             kld(col(LEFT), col(RIGHT)).alias("LeftRight"),
        //             kld(col(RIGHT), col(LEFT)).alias("RightLeft"),
        //         ])
        //         .alias("KullbackLeiblerDivergence"),
        //         jsd().sqrt().alias("JensenShannonDistance"),
        //     ]);
        // unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", 256.to_string()) };
        // println!("target.slice: {}", target.slice(0, 12));
        // println!("->lazy_frame: {}", lazy_frame.clone().collect()?);
        // data_frame = data_frame.hstack(lazy_frame.collect()?.get_columns())?;
        // println!("data_frame: {data_frame}");

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
            .id(ui.auto_id_with(ID_SOURCE).with("Settings"))
            .open(&mut self.state.open_settings_window)
            .show(ui.ctx(), |ui| self.settings.show(ui));
        Window::new(format!("{SIGMA} Statistics"))
            .id(ui.auto_id_with(ID_SOURCE).with("Statistics"))
            .open(&mut self.state.open_statistics_window)
            .show(ui.ctx(), |ui| {
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
                // self.bottom(ui)
            });
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
