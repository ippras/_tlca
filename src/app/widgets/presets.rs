use crate::{app::ICON_SIZE, utils::HashedMetaDataFrame};
use egui::{
    Id, PopupCloseBehavior, Response, RichText, ScrollArea, Ui, Widget,
    containers::menu::{MenuButton, MenuConfig},
};
use egui_ext::LabeledSeparator as _;
use egui_phosphor::regular::{DATABASE, DROP};

/// Presets
pub struct PresetsWidget;

impl PresetsWidget {
    fn content(&mut self, ui: &mut Ui) {
        // IPPRAS
        ui.hyperlink_to(RichText::new("IPPRAS").heading(), "https://ippras.ru");
        ui.menu_button("Microalgae", |ui| {
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                use crate::presets::ippras::fa::*;

                ui.labeled_separator(RichText::new("C-108 (Chromochloris zofingiensis)").heading());
                preset(ui, &C108_N);
                ui.labeled_separator(RichText::new("C-1210 (Neochlorella semenenkoi)").heading());
                preset(ui, &C1210_N);
                ui.labeled_separator(RichText::new("C-1540 (Lobosphaera sp.)").heading());
                preset(ui, &C1540_N);
                ui.labeled_separator(RichText::new("H-242 (Vischeria punctata)").heading());
                // preset(ui, &H242_CONTROL);
                ui.labeled_separator(RichText::new("H-626 (Coelastrella affinis)").heading());
                preset(ui, &H626_N);
                ui.labeled_separator(RichText::new("P-519 (Porphyridium purpureum)").heading());
                preset(ui, &P519_N);
            });
            ui.menu_button((DROP, "Triacylglycerols"), |ui| {
                use crate::presets::ippras::tag::*;

                ui.labeled_separator(RichText::new("C-108 (Chromochloris zofingiensis)").heading());
                preset(ui, &C108_N);
                ui.labeled_separator(RichText::new("C-1210 (Neochlorella semenenkoi)").heading());
                preset(ui, &C1210_N);
                ui.labeled_separator(RichText::new("C-1540 (Lobosphaera sp.)").heading());
                preset(ui, &C1540_N);
                ui.labeled_separator(RichText::new("H-242 (Vischeria punctata)").heading());
                ui.labeled_separator(RichText::new("H-626 (Coelastrella affinis)").heading());
                preset(ui, &H626_N);
                ui.labeled_separator(RichText::new("P-519 (Porphyridium purpureum)").heading());
                preset(ui, &P519_N);
            });
        });
        ui.menu_button("Sidorov (2014)", |ui| {
            doi("10.1007/s11746-014-2553-8").ui(ui);
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                use crate::presets::sidorov2014::fa::*;

                ui.labeled_separator(RichText::new("Subgenus Euonymus").heading());
                ui.labeled_separator(RichText::new("Section Euonymus").heading());
                preset(ui, &EUONYMUS_BUNGEANUS);
                preset(ui, &EUONYMUS_EUROPAEUS);
                preset(ui, &EUONYMUS_HAMILTONIANUS);
                preset(ui, &EUONYMUS_PHELLOMANUS);
                preset(ui, &EUONYMUS_SEMIEXSERTUS);
                preset(ui, &EUONYMUS_SIEBOLDIANUS);
                ui.labeled_separator(RichText::new("Section Melanocarya").heading());
                preset(ui, &EUONYMUS_ALATUS);
                preset(ui, &EUONYMUS_SACROSANCTUS);
                ui.labeled_separator(RichText::new("Section Pseudovyenomus").heading());
                preset(ui, &EUONYMUS_PAUCIFLORUS);
                ui.labeled_separator(RichText::new("Subgenus Kalonymus").heading());
                preset(ui, &EUONYMUS_LATIFOLIUS);
                preset(ui, &EUONYMUS_MACROPTERUS);
                preset(ui, &EUONYMUS_MAXIMOWICZIANUS);
                preset(ui, &EUONYMUS_SACHALINENSIS);
            });
            ui.menu_button((DROP, "Triacylglycerols"), |ui| {
                use crate::presets::sidorov2014::tag::*;

                // ui.labeled_separator(RichText::new("Subgenus Euonymus").heading());
                // ui.labeled_separator(RichText::new("Section Euonymus").heading());
                // preset(ui, &EUONYMUS_BUNGEANUS);
                // preset(ui, &EUONYMUS_EUROPAEUS);
                // preset(ui, &EUONYMUS_HAMILTONIANUS);
                // preset(ui, &EUONYMUS_PHELLOMANUS);
                // preset(ui, &EUONYMUS_SEMIEXSERTUS);
                // preset(ui, &EUONYMUS_SIEBOLDIANUS);
                // ui.labeled_separator(RichText::new("Section Melanocarya").heading());
                // preset(ui, &EUONYMUS_ALATUS);
                // preset(ui, &EUONYMUS_SACROSANCTUS);
                // ui.labeled_separator(RichText::new("Section Pseudovyenomus").heading());
                // preset(ui, &EUONYMUS_PAUCIFLORUS);
                // ui.labeled_separator(RichText::new("Subgenus Kalonymus").heading());
                // preset(ui, &EUONYMUS_LATIFOLIUS);
                // preset(ui, &EUONYMUS_MACROPTERUS);
                // preset(ui, &EUONYMUS_MAXIMOWICZIANUS);
                // preset(ui, &EUONYMUS_SACHALINENSIS);
            });
        });
        ui.menu_button("Sidorov (2025)", |ui| {
            doi("10.3390/plants14040612").ui(ui);
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                use crate::presets::sidorov2014::fa::*;

                ui.labeled_separator(RichText::new("Subgenus Euonymus").heading());

            });
            ui.menu_button((DROP, "Triacylglycerols"), |ui| {
                use crate::presets::sidorov2014::tag::*;

            });
        });
        ui.separator();
        // Third party
        ui.heading("Third party");
        ui.menu_button("Reske (1997)", |ui| {
            doi("10.1007/s11746-997-0016-1").ui(ui);
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                use crate::presets::reske1997::fa::*;

                ui.labeled_separator(RichText::new("Sunflower").heading());
                preset(ui, &SUNFLOWER_SEED_COMMODITY);
                preset(ui, &SUNFLOWER_SEED_HIGH_LINOLEIC);
                preset(ui, &SUNFLOWER_SEED_HIGH_OLEIC);
                preset(ui, &SUNFLOWER_SEED_HIGH_PALMITIC_HIGH_LINOLEIC);
                preset(ui, &SUNFLOWER_SEED_HIGH_PALMITIC_HIGH_OLEIC);
                preset(ui, &SUNFLOWER_SEED_HIGH_STEARIC_HIGH_OLEIC);
            });
            // ui.menu_button((DROP, "Triacylglycerols"), |ui| {
            //     use crate::presets::reske1997::tag::*;
            // });
        });
        ui.menu_button("Martinez-Force (2004)", |ui| {
            doi("10.1016/j.ab.2004.07.019").ui(ui);
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                use crate::presets::martínez_force2004::fa::*;

                ui.labeled_separator(RichText::new("Hazelnut").heading());
                preset(ui, &HAZELNUT);
                ui.labeled_separator(RichText::new("Olive").heading());
                preset(ui, &OLIVE);
                ui.labeled_separator(RichText::new("Rice").heading());
                preset(ui, &RICE);
                ui.labeled_separator(RichText::new("Soybean").heading());
                preset(ui, &SOYBEAN);
                ui.labeled_separator(RichText::new("Sunflower").heading());
                preset(ui, &SUNFLOWER_CAS3);
                preset(ui, &SUNFLOWER_RHA274);
                ui.labeled_separator(RichText::new("Walnut").heading());
                preset(ui, &WALNUT);
            });
            // ui.menu_button((DROP, "Triacylglycerols"), |ui| {
            //     use crate::presets::martínez_force2004::tag::*;

            //     ui.labeled_separator(RichText::new("Hazelnut").heading());
            //     preset(ui, &HAZELNUT);
            //     ui.labeled_separator(RichText::new("Olive").heading());
            //     preset(ui, &OLIVE);
            //     ui.labeled_separator(RichText::new("Rice").heading());
            //     preset(ui, &RICE);
            //     ui.labeled_separator(RichText::new("Soybean").heading());
            //     preset(ui, &SOYBEAN);
            //     ui.labeled_separator(RichText::new("Sunflower").heading());
            //     preset(ui, &SUNFLOWER_CAS3);
            //     preset(ui, &SUNFLOWER_RHA274);
            //     ui.labeled_separator(RichText::new("Walnut").heading());
            //     preset(ui, &WALNUT);
            // });
        });
    }
}

impl Widget for PresetsWidget {
    fn ui(mut self, ui: &mut Ui) -> Response {
        MenuButton::new(RichText::new(DATABASE).size(ICON_SIZE))
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| self.content(ui));
            })
            .0
    }
}

fn preset(ui: &mut Ui, frame: &HashedMetaDataFrame) {
    let title = frame.meta.format(" ");
    if ui.button(format!("{DATABASE} {title}")).clicked() {
        ui.data_mut(|data| data.insert_temp(Id::new("Data"), vec![frame.clone()]));
    }
}

fn doi(doi: &str) -> impl Fn(&mut Ui) -> Response {
    move |ui| {
        ui.hyperlink_to(
            RichText::new(format!("DOI: {doi}")).heading(),
            format!("https://doi.org/{doi}"),
        )
    }
}
