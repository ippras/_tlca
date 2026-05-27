use egui::{Label, Response, RichText, Sense, TextStyle, Ui, Widget};
use egui_phosphor::regular::{COPYRIGHT, GITHUB_LOGO, GLOBE, WARNING};

const ABBREVIATION: &str = "TLCA";
const AFFILIATION: &str = "K. A. Timiryazev Institute of Plant Physiology, Russian Academy of Sciences, Botanicheskaya Street 35, 127276 Moscow, Russia";
const NAME: &str = "Triacylglycerol List Comparator Application";

/// About widget
#[derive(Debug, Default)]
pub(crate) struct About;

impl Widget for About {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            let version = env!("CARGO_PKG_VERSION");
            ui.heading(format!("{ABBREVIATION} {version}"));
            ui.label(NAME);
            // Links
            ui.separator();
            ui.collapsing(RichText::new("Links").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(GLOBE).heading())
                        .on_hover_text("web");
                    ui.hyperlink_to(
                        "https://ippras.github.io/tlca",
                        "https://ippras.github.io/tlca",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(GITHUB_LOGO).heading())
                        .on_hover_text("github.com");
                    ui.hyperlink_to(
                        "https://github.com/ippras/tlca",
                        "https://github.com/ippras/tlca",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(WARNING).heading())
                        .on_hover_text("report an issue");
                    ui.hyperlink_to(
                        "https://github.com/ippras/tlca/issues",
                        "https://github.com/ippras/tlca/issues",
                    );
                });
            });
            // Dedications
            ui.collapsing(RichText::new("Dedications").heading(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Giorgi Kazakov:");
                    ui.label("Моим родителям, Тане и Володе, посвящается.");
                });
            });
            // Copyright
            ui.separator();
            ui.horizontal(|ui| {
                let width = ui.fonts_mut(|fonts| {
                    fonts.glyph_width(&TextStyle::Body.resolve(ui.style()), ' ')
                });
                ui.spacing_mut().item_spacing.x = width;
                ui.label(COPYRIGHT);
                ui.label("2024");
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add(Label::new("Giorgi Kazakov").sense(Sense::click()));
                ui.spacing_mut().item_spacing.x = width;
                ui.label(",");
                ui.add(Label::new("Roman Sidorov").sense(Sense::click()));
            });
        })
        .response
    }
}
