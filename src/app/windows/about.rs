use crate::{
    app::ICON_SIZE,
    r#const::{self, ABOUT, AUTHORS, LINKS, REPORT_AN_ISSUE, WEB_PAGE_ADDRESS},
};
use egui::{Context, Label, RichText, Sense, Widget, Window};
use egui_phosphor::regular::{COPYRIGHT, GITHUB_LOGO, GLOBE, INFO, WARNING};

pub(crate) const ABBREVIATION: &str = "TLCA";
pub(crate) const AFFILIATION: &str = "K. A. Timiryazev Institute of Plant Physiology, Russian Academy of Sciences, Botanicheskaya Street 35, 127276 Moscow, Russia";
pub(crate) const NAME: &str = "Triacylglycerol List Comparator Application";

/// About
#[derive(Debug, Default)]
pub(crate) struct About {
    pub(crate) open: bool,
}

impl About {
    pub(crate) fn window(&mut self, ctx: &Context) {
        Window::new(format!("{INFO} {ABOUT}"))
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let version = env!("CARGO_PKG_VERSION");
                    ui.label(RichText::new(format!("{ABBREVIATION} {version}")).heading());
                    ui.label(NAME);
                    ui.label(format!("{COPYRIGHT} 2024"));
                    ui.separator();
                    ui.heading(AUTHORS);
                    let authors = env!("CARGO_PKG_AUTHORS");
                    for author in authors.split(':') {
                        Label::new(author).sense(Sense::click()).ui(ui);
                    }
                    ui.separator();
                    ui.heading(r#const::AFFILIATION);
                    Label::new(AFFILIATION).wrap().ui(ui);
                    ui.separator();
                    ui.collapsing(RichText::new(LINKS).heading(), |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GLOBE).size(ICON_SIZE))
                                .on_hover_text(WEB_PAGE_ADDRESS);
                            ui.hyperlink_to(
                                "https://ippras.github.io/tlca",
                                "https://ippras.github.io/tlca",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GITHUB_LOGO).size(ICON_SIZE))
                                .on_hover_text("github.com");
                            ui.hyperlink_to(
                                "https://github.com/ippras/tlca",
                                "https://github.com/ippras/tlca",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(WARNING).size(ICON_SIZE))
                                .on_hover_text(REPORT_AN_ISSUE);
                            ui.hyperlink_to(
                                "https://github.com/ippras/tlca/issues",
                                "https://github.com/ippras/tlca/issues",
                            );
                        });
                    });
                });
            });
    }
}
