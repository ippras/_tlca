use crate::utils::Hashed;
use egui::{
    CentralPanel, Color32, Frame, Grid, Id, Label, RichText, ScrollArea, Sense, Stroke,
    TopBottomPanel, Ui, menu::bar,
};
use egui_dnd::dnd;
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{ARROWS_OUT_CARDINAL, CHECK, TRASH, UNITE};
use metadata::{MetaDataFrame, egui::MetadataWidget};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Data {
    pub(crate) frames: Vec<Hashed<MetaDataFrame>>,
    pub(crate) selected: HashSet<Hashed<MetaDataFrame>>,
}

impl Data {
    pub(crate) fn selected(&self) -> Vec<Hashed<MetaDataFrame>> {
        self.frames
            .iter()
            .filter_map(|frame| self.selected.contains(frame).then_some(frame.clone()))
            .collect()
    }

    pub(crate) fn add(&mut self, frame: Hashed<MetaDataFrame>) {
        if !self.frames.contains(&frame) {
            self.frames.push(frame);
        }
    }
}

impl Data {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        TopBottomPanel::top(ui.auto_id_with("LeftPane")).show_inside(ui, |ui| {
            bar(ui, |ui| {
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
                .button(RichText::new(UNITE).heading())
                .on_hover_text("unite")
                .clicked()
            {
                ui.data_mut(|data| data.insert_temp(Id::new("Unite"), frames));
            }
        });
        ui.separator();
    }

    fn central(&mut self, ui: &mut Ui) {
        dnd(ui, ui.next_auto_id()).show_vec(&mut self.frames, |ui, frame, handle, state| {
            ui.horizontal(|ui| {
                let mut checked = self.selected.contains(frame);
                if ui.checkbox(&mut checked, "").changed() {
                    if checked {
                        self.selected.insert(frame.clone());
                    } else {
                        self.selected.remove(frame);
                    }
                }
                handle.ui(ui, |ui| {
                    let text = frame.meta.format(" ").to_string();
                    ui.add(Label::new(text).truncate()).on_hover_ui(|ui| {
                        Grid::new(ui.next_auto_id()).show(ui, |ui| {
                            ui.label("Rows");
                            ui.label(frame.data.height().to_string());
                        });
                    });
                });
            });
        });
    }
}
