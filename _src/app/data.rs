use crate::utils::HashedMetaDataFrame;
use egui::{Frame, Id, Label, MenuBar, RichText, ScrollArea, TopBottomPanel, Ui};
use egui_dnd::dnd;
use egui_phosphor::regular::{CHECK, DOTS_SIX_VERTICAL, INTERSECT_THREE, PLUS, TRASH};
use metadata::egui::MetadataWidget;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub fatty_acids: VecAndHashSet,
    pub triacylglycerols: VecAndHashSet,
}

impl Data {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.visuals_mut().collapsing_header_frame = true;
        ui.collapsing(RichText::new("FattyAcids").heading(), |ui| {
            TopBottomPanel::top(
                ui.auto_id_with("LeftPane")
                    .with("TopPane")
                    .with("FattyAcids"),
            )
            .show_inside(ui, |ui| {
                MenuBar::new().ui(ui, |ui| {
                    self.fatty_acids.top(ui, "FattyAcids");
                });
            });
            Frame::central_panel(ui.style()).show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        self.fatty_acids.central(ui, "FattyAcids");
                    });
            });
        });
        ui.collapsing(RichText::new("Triacylglycerols").heading(), |ui| {
            TopBottomPanel::top(
                ui.auto_id_with("LeftPane")
                    .with("TopPane")
                    .with("Triacylglycerols"),
            )
            .show_inside(ui, |ui| {
                MenuBar::new().ui(ui, |ui| {
                    self.triacylglycerols.top(ui, "Triacylglycerols");
                });
            });
            Frame::central_panel(ui.style()).show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        self.triacylglycerols.central(ui, "Triacylglycerols");
                    });
            });
        });
    }
}

/// Vec and HashSet
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VecAndHashSet {
    pub frames: Vec<HashedMetaDataFrame>,
    pub selected: HashSet<HashedMetaDataFrame>,
}

impl VecAndHashSet {
    pub fn add(&mut self, frame: HashedMetaDataFrame) {
        if !self.frames.contains(&frame) {
            self.frames.push(frame);
        }
    }

    fn selected(&self) -> Vec<HashedMetaDataFrame> {
        self.frames
            .iter()
            .filter_map(|frame| self.selected.contains(frame).then_some(frame.clone()))
            .collect()
    }

    fn top(&mut self, ui: &mut Ui, id: impl Hash) {
        self.check(ui);
        ui.separator();
        self.delete(ui);
        ui.separator();
        self.join(ui, id);
        ui.separator();
    }

    fn check(&mut self, ui: &mut Ui) {
        if ui
            .button(RichText::new(CHECK).heading())
            .on_hover_text("Toggle")
            .on_hover_text("Toggle.hover")
            .clicked()
        {
            if self.selected.is_empty() {
                self.selected = self.frames.iter().cloned().collect();
            } else {
                self.selected.clear();
            }
        }
    }

    fn delete(&mut self, ui: &mut Ui) {
        ui.add_enabled_ui(!self.selected.is_empty(), |ui| {
            if ui
                .button(RichText::new(TRASH).heading())
                .on_hover_text("Delete")
                .on_hover_text("Delete.hover")
                .clicked()
            {
                self.frames.retain(|frame| !self.selected.remove(frame));
            }
        });
    }

    fn join(&mut self, ui: &mut Ui, id: impl Hash) {
        ui.add_enabled_ui(!self.selected.is_empty(), |ui| {
            if ui
                .button(RichText::new(INTERSECT_THREE).heading())
                .on_hover_text("Join")
                .clicked()
            {
                ui.data_mut(|data| data.insert_temp(Id::new("Join").with(id), self.selected()));
            }
        });
    }

    fn central(&mut self, ui: &mut Ui, id: impl Hash) {
        dnd(ui, ui.auto_id_with(id)).show_vec(&mut self.frames, |ui, frame, handle, _state| {
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
                ui.add(Label::new(text).truncate())
                    .on_hover_ui(|ui| MetadataWidget::new(&frame.meta).show(ui));
            });
        });
    }
}
