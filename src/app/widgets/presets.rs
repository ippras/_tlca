use crate::{
    app::{ICON_SIZE, panes::Pane},
    presets::*,
};
use anyhow::Result;
use egui::{Response, RichText, ScrollArea, Separator, Ui, Widget};
use egui_phosphor::regular::DATABASE;
use egui_tiles::Tree;
use egui_tiles_ext::{TreeExt, VERTICAL};
use metadata::MetaDataFrame;
use std::fs::File;

/// Presets
pub(crate) struct PresetsWidget<'a> {
    tree: &'a mut Tree<Pane>,
}
impl<'a> PresetsWidget<'a> {
    pub(crate) fn new(tree: &'a mut Tree<Pane>) -> Self {
        Self { tree }
    }
}

impl PresetsWidget<'_> {
    fn content(&mut self, ui: &mut Ui) {
        macro preset($frame:path) {
            let title = $frame.meta.format(" ");
            if ui
                .button(RichText::new(format!("{DATABASE} {title}")).heading())
                .clicked()
            {
                self.tree.insert_pane::<VERTICAL>(Pane::new($frame.clone()));
            }
        }

        // IPPRAS
        ui.horizontal(|ui| {
            ui.hyperlink_to(RichText::new("IPPRAS").heading(), "https://ippras.ru");
            ui.add(Separator::default().horizontal());
        });
        preset!(ippras::LOBOSPHERA_N_1);
        preset!(ippras::LOBOSPHERA_N_2);
        preset!(ippras::LOBOSPHERA_N_3);
        ui.separator();
    }
}

impl Widget for PresetsWidget<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.menu_button(RichText::new(DATABASE).size(ICON_SIZE), |ui| {
            ScrollArea::new([false, true]).show(ui, |ui| self.content(ui));
        })
        .response
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

fn ipc(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    let file = File::create(name)?;
    MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_ipc(file)?;
    Ok(())
}

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
