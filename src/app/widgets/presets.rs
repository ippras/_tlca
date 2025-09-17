use crate::{
    app::{HashedMetaDataFrame, ICON_SIZE},
    presets::*,
};
use anyhow::Result;
use egui::{
    Id, PopupCloseBehavior, Response, RichText, ScrollArea, Separator, Ui, Widget,
    containers::menu::{MenuConfig, SubMenuButton},
};
use egui_ext::LabeledSeparator as _;
use egui_phosphor::regular::DATABASE;
use metadata::MetaDataFrame;
use std::fs::File;

/// Presets
pub struct PresetsWidget;

impl PresetsWidget {
    fn content(&mut self, ui: &mut Ui) {
        // IPPRAS
        ui.hyperlink_to(RichText::new("IPPRAS").heading(), "https://ippras.ru");
        SubMenuButton::new("Microalgae")
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ui.labeled_separator(RichText::new("C-108 (Chromochloris zofingiensis)").heading());
                preset(ui, &ippras::C108_N);
                ui.labeled_separator(RichText::new("C-1210 (Neochlorella semenenkoi)").heading());
                preset(ui, &ippras::C1210_N);
                ui.labeled_separator(RichText::new("C-1540 (Lobosphaera sp.)").heading());
                preset(ui, &ippras::C1540_N);
                // ui.labeled_separator(RichText::new("H-242 (Vischeria punctata)").heading());
                ui.labeled_separator(RichText::new("H-626 (Coelastrella affinis)").heading());
                preset(ui, &ippras::H626_N);
                ui.labeled_separator(RichText::new("P-519 (Porphyridium purpureum)").heading());
                preset(ui, &ippras::P519_N);
            });
        // ui.separator();
    }
}

impl Widget for PresetsWidget {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.menu_button(RichText::new(DATABASE).size(ICON_SIZE), |ui| {
            ScrollArea::new([false, true]).show(ui, |ui| self.content(ui));
        })
        .response
    }
}

fn preset(ui: &mut Ui, frame: &HashedMetaDataFrame) {
    let title = frame.meta.format(" ");
    if ui.button(format!("{DATABASE} {title}")).clicked() {
        ui.data_mut(|data| data.insert_temp(Id::new("Data"), frame.clone()));
    }
}

fn doi_separator(doi: &str) -> impl Fn(&mut Ui) -> Response {
    move |ui| {
        ui.horizontal(|ui| {
            ui.hyperlink_to(
                RichText::new(format!("DOI: {doi}")).heading(),
                format!("https://doi.org/{doi}"),
            );
            ui.add(Separator::default().horizontal());
        })
        .response
    }
}

fn parquet(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    let file = File::create(name)?;
    MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_parquet(file)?;
    Ok(())
}

// fn ipc(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
//     let file = File::create(name)?;
//     MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_ipc(file)?;
//     Ok(())
// }

// fn ron(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
//     let file = File::create(name)?;
//     ron::ser::to_writer_pretty(
//         file,
//         &frame.data,
//         PrettyConfig::default().extensions(Extensions::IMPLICIT_SOME),
//     )?;
//     Ok(())
// }

// fn json(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
//     // let contents = ron::ser::to_string_pretty(
//     //     &frame.data,
//     //     PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_NEWTYPES),
//     // )?;
//     let contents = serde_json::to_string(&frame.data)?;
//     println!("contents: {contents}");
//     std::fs::write(name, contents)?;
//     Ok(())
// }
