use crate::{
    app::{
        panes::{MARGIN, metrics::Sign},
        states::fatty_acids::{ID_SOURCE, settings::Settings},
    },
    r#const::{EM_DASH, MEAN, SAMPLE, STANDARD_DEVIATION},
};
use egui::{Id, TextStyle, TextWrapMode, Ui, WidgetText};
use egui_extras::{Column, TableBuilder};
use egui_l20n::prelude::*;
use polars::prelude::*;
use polars_utils::format_list;
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
                for name in self.data_frame.get_column_names_str() {
                    row.col(|ui| {
                        ui.heading(name);
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
        let mean_series = self.data_frame[column].struct_()?.field_by_name(MEAN)?;
        let mean = mean_series.f64()?.get(row);
        let standard_deviation_series = self.data_frame[column]
            .struct_()?
            .field_by_name(STANDARD_DEVIATION)?;
        let standard_deviation = standard_deviation_series.f64()?.get(row);
        let text = match mean {
            Some(mean) => {
                let sign = Sign::from(mean);
                let mut color = ui.style().visuals.text_color();
                if self.settings.metric.is_finite() {
                    if self.settings.chaddock {
                        color = sign.chaddock().color(color);
                    } else {
                        color = sign.color(color);
                    }
                }
                if self.settings.standard_deviation
                    && let Some(standard_deviation) = standard_deviation
                {
                    WidgetText::from(format!("{mean} ±{standard_deviation}"))
                } else {
                    WidgetText::from(mean.to_string())
                }
                .color(color)
            }
            None => WidgetText::from(EM_DASH),
        };
        let mut response = ui.label(text);
        if response.hovered() {
            // Standard deviation
            if let Some(standard_deviation) = standard_deviation {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(STANDARD_DEVIATION));
                    ui.label(format!("±{standard_deviation}"));
                });
            }
            // Sample
            let sample_series = self.data_frame[column].struct_()?.field_by_name(SAMPLE)?;
            if let Some(sample) = sample_series.array()?.get_as_series(row) {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(SAMPLE));
                    ui.label(format_list!(sample.iter()));
                });
            }
        }
        Ok(())
    }
}
