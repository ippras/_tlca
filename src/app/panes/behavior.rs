use super::{MARGIN, Pane};
use egui::{
    CentralPanel, Frame, Margin, RichText, ScrollArea, TextStyle, TopBottomPanel, Ui, WidgetText,
    menu::bar,
};
use egui_phosphor::regular::X;
use egui_tiles::{TileId, UiResponse};

/// Behavior
#[derive(Debug)]
pub(crate) struct Behavior {
    pub(crate) close: Option<TileId>,
}

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        let response = TopBottomPanel::top(ui.auto_id_with("Pane"))
            .show_inside(ui, |ui| {
                bar(ui, |ui| {
                    ScrollArea::horizontal()
                        .show(ui, |ui| {
                            ui.set_height(
                                ui.text_style_height(&TextStyle::Heading) + 4.0 * MARGIN.y,
                            );
                            ui.visuals_mut().button_frame = false;
                            if ui.button(RichText::new(X).heading()).clicked() {
                                self.close = Some(tile_id);
                            }
                            ui.separator();
                            pane.top(ui)
                        })
                        .inner
                })
                .inner
            })
            .inner;
        CentralPanel::default()
            .frame(Frame::central_panel(&ui.style()))
            .show_inside(ui, |ui| {
                pane.central(ui);
            });
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
