use super::{MARGIN, Pane, calculation::state::State};
use egui::{
    CentralPanel, Frame, Id, MenuBar, RichText, ScrollArea, TextStyle, TopBottomPanel, Ui,
    WidgetText,
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
        let mut state = State::load(ui.ctx(), Id::new(tile_id));
        let response = TopBottomPanel::top(ui.auto_id_with("Pane"))
            .show_inside(ui, |ui| {
                MenuBar::new()
                    .ui(ui, |ui| {
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
                                pane.top(ui, &mut state)
                            })
                            .inner
                    })
                    .inner
            })
            .inner;
        CentralPanel::default()
            .frame(Frame::central_panel(&ui.style()))
            .show_inside(ui, |ui| {
                pane.central(ui, &mut state);
            });
        if let Some(id) = self.close {
            state.remove(ui.ctx(), Id::new(id));
        } else {
            state.store(ui.ctx(), Id::new(tile_id));
        }
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
