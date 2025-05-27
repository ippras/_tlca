use egui::{DragValue, InnerResponse, RichText, Ui, vec2};
use polars::prelude::*;

/// Float widget
pub(crate) struct FloatWidget {
    pub(crate) value: Option<f64>,
    pub(crate) editable: bool,
    pub(crate) hover: bool,
    pub(crate) precision: Option<usize>,
}

impl FloatWidget {
    pub(crate) fn new(value: Option<f64>) -> Self {
        Self {
            value,
            editable: false,
            hover: false,
            precision: None,
        }
    }

    pub(crate) fn editable(self, editable: bool) -> Self {
        Self { editable, ..self }
    }

    pub(crate) fn hover(self) -> Self {
        Self {
            hover: true,
            ..self
        }
    }

    pub(crate) fn precision(self, precision: Option<usize>) -> Self {
        Self { precision, ..self }
    }

    pub(crate) fn show(self, ui: &mut Ui) -> InnerResponse<Option<f64>> {
        let format = |value: f64| match self.precision {
            Some(precision) => format!("{value:.precision$}"),
            None => AnyValue::from(value).to_string(),
        };
        let mut inner = None;
        // None
        let Some(mut value) = self.value else {
            let mut response = ui.label("None");
            if self.editable {
                let mut changed = false;
                response.context_menu(|ui| {
                    if ui.button("Some").clicked() {
                        inner = Some(Default::default());
                        changed = true;
                        ui.close_menu();
                    }
                });
                if changed {
                    response.mark_changed();
                };
            }
            return InnerResponse::new(inner, response);
        };
        // Some
        let text = format(value);
        // Editable
        let mut response = if self.editable {
            // Writable
            let mut response = ui.add_sized(
                vec2(ui.available_width(), ui.style().spacing.interact_size.y),
                DragValue::new(&mut value)
                    .range(0.0..=f64::MAX)
                    .custom_formatter(|value, _| format(value)),
            );
            if response.changed() {
                inner = Some(value);
            };
            let mut changed = false;
            response.context_menu(|ui| {
                if ui.button("None").clicked() {
                    inner = None;
                    changed = true;
                    ui.close_menu();
                }
            });
            if changed {
                response.mark_changed();
            };
            response
        } else {
            // Readable
            ui.label(text)
        };
        if self.hover {
            response = response.on_hover_text(RichText::new(AnyValue::Float64(value).to_string()));
        }
        InnerResponse::new(inner, response)
    }
}
