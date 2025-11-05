use crate::{app::ICON_SIZE, utils::HashedMetaDataFrame};
use egui::{
    Align2, Color32, FontId, Id, PopupCloseBehavior, Response, RichText, ScrollArea, Sense, Ui,
    Widget,
    containers::menu::{MenuButton, MenuConfig},
    vec2,
};
use egui_ext::LabeledSeparator as _;
use egui_phosphor::regular::{DATABASE, DROP};

/// Presets widget
pub struct Presets;

impl Presets {
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
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
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
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
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
                use crate::presets::sidorov2025::fa::*;

                ui.labeled_separator(RichText::new("Lunaria rediviva").heading());
                preset(ui, &LUNARIA_REDIVIVA_FISCHER);
                preset(ui, &LUNARIA_REDIVIVA_SAPONIFICATION);
                preset(ui, &LUNARIA_REDIVIVA_TMSH);
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                use crate::presets::sidorov2025::tag::*;
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
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                use crate::presets::reske1997::tag::*;
            });
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

impl Widget for Presets {
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

struct CombinedIcon<'a> {
    base_icon: &'a str,
    base_size: Option<f32>,
    base_color: Option<Color32>,

    overlay_icon: &'a str,
    overlay_size: Option<f32>,
    overlay_color: Option<Color32>,
    overlay_offset: Option<f32>,
}

// Конструктор для удобства
impl<'a> CombinedIcon<'a> {
    pub fn new(base_icon: &'a str, overlay_icon: &'a str) -> Self {
        Self {
            base_icon,
            base_size: None,
            base_color: None,
            overlay_icon,
            overlay_size: None,
            overlay_color: Some(Color32::from_rgb(0, 180, 0)),
            overlay_offset: None,
        }
    }

    // Методы-конструкторы для кастомизации
    pub fn base_size(mut self, size: f32) -> Self {
        self.base_size = Some(size);
        self
    }

    pub fn overlay_size(mut self, size: f32) -> Self {
        self.overlay_size = Some(size);
        self
    }

    pub fn base_color(mut self, color: Color32) -> Self {
        self.base_color = Some(color);
        self
    }

    pub fn overlay_color(mut self, color: Color32) -> Self {
        self.overlay_color = Some(color);
        self
    }
}

impl<'a> Widget for CombinedIcon<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Определяем желаемый размер виджета
        let base_size = self.base_size.unwrap_or(ui.spacing().icon_width);
        let overlay_size = self.overlay_size.unwrap_or(ui.spacing().icon_width);
        let desired_size = vec2(base_size, base_size);

        // Выделяем место в макете
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Рисуем только если виджет видим на экране (оптимизация)
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let visuals = ui.style().interact(&response);

            // Определяем цвет для базовой иконки: либо заданный, либо из стиля
            let base_color = self.base_color.unwrap_or(visuals.text_color());
            let overlay_color = self.overlay_color.unwrap_or(visuals.text_color());

            // Рисуем базовый значок
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.base_icon,
                FontId::proportional(base_size),
                base_color,
            );

            // Рисуем накладываемый значок
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.overlay_icon,
                FontId::proportional(overlay_size),
                overlay_color,
            );
        }

        response
    }
}
