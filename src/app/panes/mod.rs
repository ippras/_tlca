use crate::{
    r#const::{EM_DASH, MEAN, SAMPLE, STANDARD_DEVIATION},
    utils::HashedMetaDataFrame,
};
use egui::{Response, TextWrapMode, Ui, Vec2, WidgetText, vec2};
use egui_l20n::prelude::*;
use egui_tiles::{TileId, UiResponse};
use polars::prelude::*;

const MARGIN: Vec2 = vec2(4.0, 2.0);

/// Pane
pub(crate) enum Pane {
    FattyAcids(fatty_acids::Pane),
    Triacylglycerols(triacylglycerols::Pane),
}

impl Pane {
    pub(crate) fn fatty_acids(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self::FattyAcids(fatty_acids::Pane::new(frames))
    }

    pub(crate) fn triacylglycerols(frames: Vec<HashedMetaDataFrame>) -> Self {
        Self::Triacylglycerols(triacylglycerols::Pane::new(frames))
    }
}

/// Behavior
#[derive(Debug)]
pub(crate) struct Behavior {
    pub(crate) close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        match pane {
            Pane::Triacylglycerols(pane) => pane.title().into(),
            Pane::FattyAcids(pane) => pane.title().into(),
        }
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        match pane {
            Pane::FattyAcids(pane) => pane.ui(ui, self, tile_id),
            Pane::Triacylglycerols(pane) => pane.ui(ui, self, tile_id),
        }
    }
}

pub(crate) mod fatty_acids;
pub(crate) mod triacylglycerols;

mod metrics;
