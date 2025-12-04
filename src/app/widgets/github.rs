use crate::{
    app::ICON_SIZE,
    utils::{HashedDataFrame, HashedMetaDataFrame, spawn},
};
use anyhow::{Context as _, Error, Result};
use egui::{
    Context, Id, PopupCloseBehavior, Response, RichText, ScrollArea, Ui, Widget,
    containers::menu::{MenuButton, MenuConfig},
};
use egui_ext::Doi as _;
use egui_phosphor::regular::{CLOUD_ARROW_DOWN, DROP};
use ehttp::{Request, fetch_async};
use metadata::polars::MetaDataFrame;
use std::borrow::Cow;
use tracing::{instrument, trace};
use url::Url;
use urlencoding::decode;

/// Github widget
pub struct Github;

impl Github {
    fn content(&mut self, ui: &mut Ui) {
        // IPPRAS
        ui.hyperlink_to(RichText::new("IPPRAS").heading(), "https://ippras.ru");
        ui.menu_button("Acer", |ui| {
            ui.heading("Acer");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Acer Ginnala");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer ginnala.2025-07-08.fa.utca.ron");
                ui.heading("Acer Pensylvanicum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer pensylvanicum.2025-07-08.fa.utca.ron");
                ui.heading("Acer Rubrum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer rubrum.2025-07-09.fa.utca.ron");
                ui.heading("Acer Spicatum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer spicatum.2025-07-09.fa.utca.ron");
                ui.heading("Acer Ukurunduense");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer ukurunduense.2025-07-08.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                ui.heading("Acer Ginnala");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer ginnala.2025-07-08.tag.utca.ron");
                ui.heading("Acer Pensylvanicum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer pensylvanicum.2025-07-08.tag.utca.ron");
                ui.heading("Acer Rubrum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer rubrum.2025-07-09.tag.utca.ron");
                ui.heading("Acer Spicatum");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer spicatum.2025-07-09.tag.utca.ron");
                ui.heading("Acer Ukurunduense");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Acer/Acer ukurunduense.2025-07-08.tag.utca.ron");
            });
        });
        ui.menu_button("Cedrus", |ui| {
            ui.heading("Cedrus");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Cedrus/.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Cedrus/.tag.utca.ron");
            });
        });
        ui.menu_button("Helianthus annuus", |ui| {
            ui.heading("Helianthus annuus");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2233.2025-10-29.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2699.2025-10-30.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2776.2025-11-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3384.2025-10-31.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3599.2025-10-30.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3675.2025-10-31.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3714.2025-10-31.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2233.2025-10-29.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2699.2025-10-30.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-2776.2025-11-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3384.2025-10-31.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3599.2025-10-30.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3675.2025-10-31.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/HelianthusAnnuus/К-3714.2025-10-31.tag.utca.ron");
            });
        });
        ui.menu_button("Microalgae", |ui| {
            ui.heading("Microalgae");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Chromochloris zofingiensis");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-108[-N].2025-04-23.fa.utca.ron");
                ui.heading("Neochlorella semenenkoi");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-1210[-N].2025-04-24.fa.utca.ron");
                ui.heading("Lobosphaera sp.");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-1540[-N].2025-04-24.fa.utca.ron");
                ui.heading("Vischeria punctata");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/H-242[Control].2023-10-24.fa.utca.ron");
                ui.heading("Coelastrella affinis");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/H-626[-N].2025-04-24.fa.utca.ron");
                ui.heading("Porphyridium purpureum");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/P-519[-N].2025-04-23.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                ui.heading("Chromochloris zofingiensis");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-108[-N].2025-04-23.tag.utca.ron");
                ui.heading("Neochlorella semenenkoi");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-1210[-N].2025-04-24.tag.utca.ron");
                ui.heading("Lobosphaera sp.");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/C-1540[-N].2025-04-24.tag.utca.ron");
                ui.heading("Vischeria punctata");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/H-242[Control].2023-10-24.tag.utca.ron");
                ui.heading("Coelastrella affinis");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/H-626[-N].2025-04-24.tag.utca.ron");
                ui.heading("Porphyridium purpureum");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Microalgae/P-519[-N].2025-04-23.tag.utca.ron");
            });
        });
        ui.menu_button("Sidorov (2014)", |ui| {
            ui.doi("10.1007/s11746-014-2553-8");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Subgenus Euonymus");
                ui.separator();
                ui.heading("Section Euonymus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus bungeanus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus europaeus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus hamiltonianus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus phellomanus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus semiexsertus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sieboldianus.2014-06-19.fa.utca.ron");
                ui.heading("Section Melanocarya");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus alatus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sacrosanctus.2014-06-19.fa.utca.ron");
                ui.heading("Section Pseudovyenomus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus pauciflorus.2014-06-19.fa.utca.ron");
                ui.separator();
                ui.heading("Subgenus Kalonymus");
                ui.separator();
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus latifolius.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus macropterus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus maximowiczianus.2014-06-19.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sachalinensis.2014-06-19.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                ui.heading("Subgenus Euonymus");
                ui.separator();
                ui.heading("Section Euonymus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus bungeanus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus europaeus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus hamiltonianus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus phellomanus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus semiexsertus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sieboldianus.2014-06-19.tag.utca.ron");
                ui.heading("Section Melanocarya");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus alatus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sacrosanctus.2014-06-19.tag.utca.ron");
                ui.heading("Section Pseudovyenomus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus pauciflorus.2014-06-19.tag.utca.ron");
                ui.separator();
                ui.heading("Subgenus Kalonymus");
                ui.separator();
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus latifolius.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus macropterus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus maximowiczianus.2014-06-19.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2014/Euonymus sachalinensis.2014-06-19.tag.utca.ron");
            });
        });
        ui.menu_button("Sidorov (2025)", |ui| {
            ui.doi("10.3390/plants14040612");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Lunaria Rediviva");
                // ui.separator();
                // ui.heading("TMSH");
                // ui.separator();
                // ui.heading("SN-1,2(2,3)");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2025/ron/Lunaria rediviva[TMSH;SN-1,2(2,3);1.1].2024-01-24.utca.ron");
                ui.heading("SN-2");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2025/ron/Lunaria rediviva[TMSH;SN-2;1.1].2024-01-24.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                ui.heading("Lunaria Rediviva");
                // ui.separator();
                // ui.heading("TMSH");
                // ui.separator();
                // ui.heading("SN-1,2(2,3)");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2025/ron/Lunaria rediviva[TMSH;SN-1,2(2,3);1.1].2024-01-24.utca.ron");
                // ui.heading("SN-2");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/Sidorov2025/ron/Lunaria rediviva[TMSH;SN-2;1.1].2024-01-24.utca.ron");
            });
        });
        ui.separator();
        // Third party
        ui.heading("Third party");
        ui.menu_button("Reske (1997)", |ui| {
            ui.doi("10.1007/s11746-997-0016-1");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Helianthus annuus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (Commodity).1997-08-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High linoleic).1997-08-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High oleic).1997-08-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High palmitic, high linoleic).1997-08-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High palmitic, high oleic).1997-08-01.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High stearic, high oleic).1997-08-01.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                ui.heading("Helianthus annuus");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (Commodity).1997-08-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High linoleic).1997-08-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High oleic).1997-08-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High palmitic, high linoleic).1997-08-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High palmitic, high oleic).1997-08-01.tag.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Reske1997/Sunflower seed (High stearic, high oleic).1997-08-01.tag.utca.ron");
            });
        });
        ui.menu_button("Martinez-Force (2004)", |ui| {
            ui.doi("10.1016/j.ab.2004.07.019");
            ui.menu_button((DROP, "Fatty acids"), |ui| {
                ui.heading("Hazelnut");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Hazelnut.2004-05-20.fa.utca.ron");
                ui.heading("Olive");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Olive.2004-05-20.fa.utca.ron");
                ui.heading("Rice");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Rice.2004-05-20.fa.utca.ron");
                ui.heading("Soybean");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Soybean.2004-05-20.fa.utca.ron");
                ui.heading("Sunflower");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Sunflower CAS-3.2004-05-20.fa.utca.ron");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Sunflower RHA-274.2004-05-20.fa.utca.ron");
                ui.heading("Walnut");
                _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Walnut.2004-05-20.fa.utca.ron");
            });
            ui.menu_button((DROP, DROP, DROP, "Triacylglycerols"), |ui| {
                // ui.heading("Hazelnut");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Hazelnut.2004-05-20.tag.utca.ron");
                // ui.heading("Olive");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Olive.2004-05-20.tag.utca.ron");
                // ui.heading("Rice");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Rice.2004-05-20.tag.utca.ron");
                // ui.heading("Soybean");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Soybean.2004-05-20.tag.utca.ron");
                // ui.heading("Sunflower");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Sunflower CAS-3.2004-05-20.tag.utca.ron");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Sunflower RHA-274.2004-05-20.tag.utca.ron");
                // ui.heading("Walnut");
                // _ = preset(ui, "https://raw.githubusercontent.com/ippras/utca/presets/ThirdParty/Martinez-Force2004/Walnut.2004-05-20.tag.utca.ron");
            });
        });
    }
}

impl Widget for Github {
    fn ui(mut self, ui: &mut Ui) -> Response {
        MenuButton::new(RichText::new(CLOUD_ARROW_DOWN).size(ICON_SIZE))
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| self.content(ui));
            })
            .0
    }
}

/// Preset
#[instrument(skip(ui), err)]
fn preset(ui: &mut Ui, input: &str) -> Result<()> {
    let url = Url::parse(input)?;
    let (name, date) = parse(&url)?;
    if ui.button(format!("{name} {date}")).clicked() {
        load(ui.ctx(), url);
    }
    Ok(())
}

/// Parse preset url
fn parse<'a>(url: &'a Url) -> Result<(Cow<'a, str>, &'a str)> {
    let segment = url
        .path_segments()
        .context("Preset get path segments")?
        .last()
        .context("Preset get last path segment")?;
    let input = segment
        .trim_end_matches(".fa.utca.ron")
        .trim_end_matches(".tag.utca.ron");
    let (name, date) = input
        .rsplit_once(".")
        .context("Preset parse name and date")?;
    Ok((decode(name)?, date))
}

fn load(ctx: &Context, url: Url) {
    let ctx = ctx.clone();
    _ = spawn(async move {
        if let Ok(frame) = try_load(&url).await {
            trace!(?frame);
            ctx.data_mut(|data| data.insert_temp(Id::new("Data"), vec![frame]));
        }
    });
}

#[instrument(err)]
async fn try_load(url: &Url) -> Result<HashedMetaDataFrame> {
    let request = Request::get(url);
    let response = fetch_async(request).await.map_err(Error::msg)?;
    let text = response.text().context("Try load get response text")?;
    trace!(?text);
    let frame = ron::de::from_str::<MetaDataFrame>(text)?;
    Ok(MetaDataFrame {
        meta: frame.meta,
        data: HashedDataFrame::new(frame.data)?,
    })
}
