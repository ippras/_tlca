use crate::{app::HashedMetaDataFrame, utils::Hashed};
use egui::{CentralPanel, Grid, Id, Label, MenuBar, RichText, ScrollArea, TopBottomPanel, Ui};
use egui_dnd::dnd;
use egui_phosphor::regular::{CHECK, DOTS_SIX_VERTICAL, INTERSECT_THREE, TRASH, UNITE};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::LazyLock};

pub(crate) static EMPTY_DATA_FRAME: LazyLock<Hashed<DataFrame>> = LazyLock::new(|| Hashed {
    value: DataFrame::empty(),
    hash: 0,
});

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Data {
    pub(crate) frames: Vec<HashedMetaDataFrame>,
    pub(crate) selected: HashSet<HashedMetaDataFrame>,
}

impl Data {
    pub(crate) fn selected(&self) -> Vec<HashedMetaDataFrame> {
        self.frames
            .iter()
            .filter_map(|frame| self.selected.contains(frame).then_some(frame.clone()))
            .collect()
    }

    pub(crate) fn add(&mut self, frame: HashedMetaDataFrame) {
        if !self.frames.contains(&frame) {
            self.frames.push(frame);
        }
    }
}

impl Data {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        TopBottomPanel::top(ui.auto_id_with("LeftPane")).show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                self.top(ui);
            });
        });
        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                self.central(ui);
            });
        });
    }

    fn top(&mut self, ui: &mut Ui) {
        // Delete
        ui.heading("loaded_files")
            .on_hover_text("loaded_files.hover");
        ui.separator();
        // Delete
        if ui
            .button(RichText::new(TRASH).heading())
            .on_hover_text("delete")
            .clicked()
        {
            self.frames.retain(|frame| !self.selected.remove(frame));
        }
        ui.separator();
        // Toggle
        if ui
            .button(RichText::new(CHECK).heading())
            .on_hover_text("toggle")
            .on_hover_text("toggle.hover")
            .clicked()
        {
            if self.selected.is_empty() {
                self.selected = self.frames.iter().cloned().collect();
            } else {
                self.selected.clear();
            }
        }
        ui.separator();
        // Configuration
        let frames = self.selected();
        ui.add_enabled_ui(!frames.is_empty(), |ui| {
            if ui
                .button(RichText::new(INTERSECT_THREE).heading())
                .on_hover_text("Union")
                .clicked()
            {
                ui.data_mut(|data| data.insert_temp(Id::new("Unite"), frames));
            }
        });
        ui.separator();
    }

    fn central(&mut self, ui: &mut Ui) {
        dnd(ui, ui.next_auto_id()).show_vec(&mut self.frames, |ui, frame, handle, _state| {
            ui.horizontal(|ui| {
                handle.ui(ui, |ui| {
                    ui.label(DOTS_SIX_VERTICAL);
                });
                let mut checked = self.selected.contains(frame);
                if ui.checkbox(&mut checked, "").changed() {
                    if checked {
                        self.selected.insert(frame.clone());
                    } else {
                        self.selected.remove(frame);
                    }
                }
                let text = frame.meta.format(" ").to_string();
                ui.add(Label::new(text).truncate()).on_hover_ui(|ui| {
                    Grid::new(ui.next_auto_id()).show(ui, |ui| {
                        ui.label("Rows");
                        ui.label(frame.data.height().to_string());
                    });
                });
            });
        });
    }
}
