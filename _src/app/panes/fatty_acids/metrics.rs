use crate::{
    app::{
        panes::{MARGIN, metrics::Sign},
        states::fatty_acids::{ID_SOURCE, settings::Settings},
    },
    r#const::EM_DASH,
};
use egui::{Id, TextStyle, TextWrapMode, Ui, WidgetText};
use egui_extras::{Column, TableBuilder};
use egui_l20n::prelude::*;
use polars::prelude::*;
use tracing::instrument;

/// Metrics
pub struct Metrics<'a> {
    pub data_frame: &'a DataFrame,
    pub settings: &'a Settings,
}

impl<'a> Metrics<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }
}

impl Metrics<'_> {
    #[instrument(skip_all, err)]
    pub fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Metrics");
        let height = ui.text_style_height(&TextStyle::Heading);
        let rows = self.data_frame.height();
        let columns = self.data_frame.width();
        ui.style_mut().wrap_mode = if self.settings.truncate {
            Some(TextWrapMode::Truncate)
        } else {
            Some(TextWrapMode::Extend)
        };
        TableBuilder::new(ui)
            .id_salt(id_salt)
            .striped(true)
            .resizable(true)
            .columns(Column::auto(), columns + 1)
            .header(height + 2.0 * MARGIN.y, |mut row| {
                row.col(|ui| {
                    ui.heading(ui.localize(self.settings.metric.text()));
                });
                for name in self.data_frame.get_column_names() {
                    row.col(|ui| {
                        ui.heading(name.as_str());
                    });
                }
            })
            .body(|mut body| {
                body.ui_mut().style_mut().wrap_mode = Some(TextWrapMode::Extend);
                body.rows(height, rows, |mut row| {
                    let index = row.index();
                    row.col(|ui| {
                        ui.label(self.data_frame[index].name().as_str());
                    });
                    for column in 0..columns {
                        row.col(|ui| {
                            _ = self.body_cell_content_ui(ui, index, column);
                        });
                    }
                });
            });
        Ok(())
    }

    #[instrument(skip(self, ui), err)]
    fn body_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) -> PolarsResult<()> {
        let text = match self.data_frame[column].f64()?.get(row) {
            Some(metric) => {
                let sign = Sign::from(metric);
                let mut color = ui.style().visuals.text_color();
                if self.settings.metric.is_finite() {
                    if self.settings.chaddock {
                        color = sign.chaddock().color(color);
                    } else {
                        color = sign.color(color);
                    }
                }
                WidgetText::from(metric.to_string()).color(color)
            }
            None => WidgetText::from(EM_DASH),
        };
        ui.label(text);
        Ok(())
    }
}
