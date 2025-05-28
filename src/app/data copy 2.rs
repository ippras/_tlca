use egui::{Color32, Frame, Grid, Id, Label, RichText, Sense, Stroke, Ui, menu::bar};
use egui_dnd::dnd;
use egui_extras::{Column, TableBuilder};
use egui_phosphor::regular::{ARROWS_OUT_CARDINAL, CHECK, TRASH};
use metadata::{MetaDataFrame, egui::MetadataWidget};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Data {
    pub(crate) frames: Vec<MetaDataFrame>,
    pub(crate) selected: HashSet<MetaDataFrame>,
}

impl Data {
    pub(crate) fn selected(&self) -> Vec<MetaDataFrame> {
        self.frames
            .iter()
            .filter_map(|frame| self.selected.contains(frame).then_some(frame.clone()))
            .collect()
    }

    pub(crate) fn add(&mut self, frame: MetaDataFrame) {
        self.frames.push(frame);
    }
}

impl Data {
    pub(crate) fn header(&mut self, ui: &mut Ui) {
        // Delete
        bar(ui, |ui| {
            ui.heading("loaded-files")
                .on_hover_text("loaded-files.hover");
            ui.separator();
            // Toggle all
            if ui
                .button(RichText::new(CHECK).heading())
                .on_hover_text("toggle-all")
                .on_hover_text("toggle-all.hover")
                .clicked()
            {
                if self.selected.is_empty() {
                    self.selected = self.frames.iter().cloned().collect();
                } else {
                    self.selected.clear();
                }
            }
            ui.separator();
            // Delete all
            if ui
                .button(RichText::new(TRASH).heading())
                .on_hover_text("delete-all")
                .clicked()
            {
                self.frames.retain(|frame| !self.selected.remove(frame));
                // *self = Default::default();
            }
            ui.separator();
            // // Configuration
            // let frames = self.selected();
            // ui.add_enabled_ui(!frames.is_empty(), |ui| {
            //     if ui
            //         .button(RichText::new(ConfigurationPane::icon()).heading())
            //         .on_hover_text("configuration")
            //         .clicked()
            //     {
            //         ui.data_mut(|data| data.insert_temp(Id::new("Configure"), frames));
            //     }
            // });
            ui.separator();
        });
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        // Header
        self.header(ui);
        // Body
        ui.separator();
        ui.visuals_mut().widgets.inactive.bg_fill = Color32::TRANSPARENT;
        // let mut swap = None;
        // ui.dnd_drop_zone::<usize, ()>(Frame::new(), |ui| {
        //     for (index, frame) in self.frames.iter().enumerate() {
        //         let mut changed = false;
        //         ui.horizontal(|ui| {
        //             let response = ui
        //                 .dnd_drag_source(ui.auto_id_with(index), index, |ui| {
        //                     ui.label(index.to_string())
        //                 })
        //                 .response;
        //             // Detect drops onto this item
        //             if let (Some(pointer), Some(hovered_payload)) = (
        //                 ui.input(|input| input.pointer.interact_pos()),
        //                 response.dnd_hover_payload::<usize>(),
        //             ) {
        //                 let rect = response.rect;
        //                 // Preview insertion:
        //                 let stroke = Stroke::new(1.0, Color32::WHITE);
        //                 let to = if *hovered_payload == index {
        //                     // We are dragged onto ourselves
        //                     ui.painter().hline(rect.x_range(), rect.center().y, stroke);
        //                     index
        //                 } else if pointer.y < rect.center().y {
        //                     // Above us
        //                     ui.painter().hline(rect.x_range(), rect.top(), stroke);
        //                     index
        //                 } else {
        //                     // Below us
        //                     ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
        //                     index + 1
        //                 };
        //                 if let Some(from) = response.dnd_release_payload() {
        //                     // The user dropped onto this item.
        //                     swap = Some((*from, to));
        //                 }
        //             }
        //             // Checkbox
        //             let mut checked = self.selected.contains(frame);
        //             let response = ui.checkbox(&mut checked, "");
        //             changed |= response.changed();
        //             // Label
        //             let text = frame.meta.format(" ").to_string();
        //             let response = ui
        //                 .add(Label::new(text).sense(Sense::click()).truncate())
        //                 .on_hover_ui(|ui| {
        //                     MetadataWidget::new(&frame.meta).show(ui);
        //                 })
        //                 .on_hover_ui(|ui| {
        //                     Grid::new(ui.next_auto_id()).show(ui, |ui| {
        //                         ui.label("Rows");
        //                         ui.label(frame.data.height().to_string());
        //                         ui.end_row();
        //                         ui.label("Columns");
        //                         ui.label(frame.data.width().to_string());
        //                         ui.end_row();
        //                     });
        //                 });
        //             changed |= response.clicked();
        //         });
        //         if changed {
        //             if ui.input(|input| input.modifiers.command) {
        //                 if self.selected.contains(frame) {
        //                     self.selected.remove(frame);
        //                 } else {
        //                     self.selected.insert(frame.clone());
        //                 }
        //             } else {
        //                 if self.selected.contains(frame) {
        //                     self.selected.remove(&frame);
        //                 } else {
        //                     self.selected.insert(frame.clone());
        //                 }
        //             }
        //         }
        //     }
        // });
        // if let Some((from, to)) = swap {
        //     if from != to {
        //         let frame = self.frames.remove(from);
        //         if from < to {
        //             self.frames.insert(to - 1, frame);
        //         } else {
        //             self.frames.insert(to, frame);
        //         }
        //     }
        // }
        // if let Some(index) = delete {
        //     self.selected.remove(&self.frames.remove(index));
        // }
        dnd(ui, ui.next_auto_id()).show_vec(&mut self.frames, |ui, frame, handle, state| {
            ui.horizontal(|ui| {
                handle.ui(ui, |ui| {
                    let _ = ui.label(ARROWS_OUT_CARDINAL);
                });
                ui.checkbox(&mut self.selected.contains(frame), "");
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
