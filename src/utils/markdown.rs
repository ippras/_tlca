use egui::{Id, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use parking_lot::Mutex;
use std::sync::Arc;

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn markdown_ui(&mut self, markdown: &str);
}

impl UiExt for Ui {
    fn markdown_ui(&mut self, markdown: &str) {
        let ui = self;
        let cache = ui.data_mut(|data| {
            data.get_temp_mut_or_default::<Arc<Mutex<CommonMarkCache>>>(Id::new(
                "GlobalEguiCommonmarkCache",
            ))
            .clone()
        });
        CommonMarkViewer::new().show(ui, &mut cache.lock(), markdown);
    }
}
