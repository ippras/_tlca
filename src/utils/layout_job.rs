use egui::{Color32, TextFormat, TextStyle, Ui, text::LayoutJob};

/// Extension methods for [`Ui`]
pub trait LayoutJobExt {
    fn subscripted_text(
        ui: &Ui,
        text: &str,
        style: Option<TextStyle>,
        color: Option<Color32>,
    ) -> LayoutJob;
}

impl LayoutJobExt for LayoutJob {
    fn subscripted_text(
        ui: &Ui,
        text: &str,
        style: Option<TextStyle>,
        color: Option<Color32>,
    ) -> LayoutJob {
        let (text, subscription) = text.split_once('_').unwrap_or((text, ""));
        let mut layout_job = LayoutJob::default();
        let text_color = color.unwrap_or_else(|| ui.visuals().text_color());
        let text_style = style.unwrap_or(TextStyle::Body);
        let font_id = text_style.resolve(ui.style());
        layout_job.append(text, 0.0, TextFormat::simple(font_id, text_color));
        let font_id = match text_style {
            TextStyle::Heading => TextStyle::Body.resolve(ui.style()),
            TextStyle::Body => TextStyle::Small.resolve(ui.style()),
            _ => {
                let mut font_id = text_style.resolve(ui.style());
                font_id.size /= 2.0;
                font_id
            }
        };
        layout_job.append(subscription, 0.0, TextFormat::simple(font_id, text_color));
        layout_job
    }
}
