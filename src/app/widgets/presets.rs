use crate::{app::ICON_SIZE, presets::*, utils::HashedMetaDataFrame};
use egui::{
    Id, PopupCloseBehavior, Response, RichText, ScrollArea, Separator, Ui, Widget,
    containers::menu::{MenuConfig, SubMenuButton},
};
use egui_ext::LabeledSeparator as _;
use egui_phosphor::regular::DATABASE;

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
                ui.labeled_separator(RichText::new("H-242 (Vischeria punctata)").heading());
                ui.labeled_separator(RichText::new("H-626 (Coelastrella affinis)").heading());
                preset(ui, &ippras::H626_N);
                ui.labeled_separator(RichText::new("P-519 (Porphyridium purpureum)").heading());
                preset(ui, &ippras::P519_N);
            });
        SubMenuButton::new("Sidorov (2014)")
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ui.hyperlink_to(
                    RichText::new("10.1007/s11746-014-2553-8").heading(),
                    "https://doi.org/10.1007/s11746-014-2553-8",
                );
                ui.labeled_separator(RichText::new("Subgenus Euonymus").heading());
                ui.labeled_separator(RichText::new("Section Euonymus").heading());
                preset(ui, &sidorov2014::EUONYMUS_BUNGEANUS);
                preset(ui, &sidorov2014::EUONYMUS_EUROPAEUS);
                preset(ui, &sidorov2014::EUONYMUS_HAMILTONIANUS);
                preset(ui, &sidorov2014::EUONYMUS_PHELLOMANUS);
                preset(ui, &sidorov2014::EUONYMUS_SEMIEXSERTUS);
                preset(ui, &sidorov2014::EUONYMUS_SIEBOLDIANUS);
                ui.labeled_separator(RichText::new("Section Melanocarya").heading());
                preset(ui, &sidorov2014::EUONYMUS_ALATUS);
                preset(ui, &sidorov2014::EUONYMUS_SACROSANCTUS);
                ui.labeled_separator(RichText::new("Section Pseudovyenomus").heading());
                preset(ui, &sidorov2014::EUONYMUS_PAUCIFLORUS);
                ui.labeled_separator(RichText::new("Subgenus Kalonymus").heading());
                preset(ui, &sidorov2014::EUONYMUS_LATIFOLIUS);
                preset(ui, &sidorov2014::EUONYMUS_MACROPTERUS);
                preset(ui, &sidorov2014::EUONYMUS_MAXIMOWICZIANUS);
                preset(ui, &sidorov2014::EUONYMUS_SACHALINENSIS);
            });
        ui.separator();
        // Third party
        ui.heading("Third party");
        SubMenuButton::new("Martinez-Force 2004")
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ui.hyperlink_to(
                    RichText::new("10.1016/j.ab.2004.07.019").heading(),
                    "https://doi.org/10.1016/j.ab.2004.07.019",
                );
                ui.labeled_separator(RichText::new("Hazelnut").heading());
                preset(ui, &martínez_force2004::HAZELNUT);
                ui.labeled_separator(RichText::new("Olive").heading());
                preset(ui, &martínez_force2004::OLIVE);
                ui.labeled_separator(RichText::new("Rice").heading());
                preset(ui, &martínez_force2004::RICE);
                ui.labeled_separator(RichText::new("Soybean").heading());
                preset(ui, &martínez_force2004::SOYBEAN);
                ui.labeled_separator(RichText::new("Sunflower").heading());
                preset(ui, &martínez_force2004::SUNFLOWER_CAS3);
                preset(ui, &martínez_force2004::SUNFLOWER_RHA274);
                ui.labeled_separator(RichText::new("Walnut").heading());
                preset(ui, &martínez_force2004::WALNUT);
            });
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

// fn parquet(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
//     let file = File::create(name)?;
//     MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_parquet(file)?;
//     Ok(())
// }

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
