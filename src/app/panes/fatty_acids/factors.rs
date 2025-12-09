use crate::{
    app::{
        panes::MARGIN,
        states::fatty_acids::{ID_SOURCE, Settings},
    },
    r#const::{EM_DASH, MEAN, SAMPLE, STANDARD_DEVIATION, THRESHOLD},
};
use egui::{Id, TextStyle, TextWrapMode, Ui, WidgetText};
use egui_extras::{Column, TableBuilder};
use egui_l20n::UiExt as _;
use egui_phosphor::regular::HASH;
use lipid::prelude::*;
use polars::prelude::*;
use polars_utils::{format_list, format_list_truncated};
use tracing::instrument;

/// Factors widget
pub(crate) struct Factors<'a> {
    pub data_frame: &'a DataFrame,
    pub settings: &'a Settings,
}

impl<'a> Factors<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }

    #[instrument(skip_all, err)]
    pub(crate) fn show(mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Factors");
        let height = ui.text_style_height(&TextStyle::Heading);
        let rows = self.data_frame.height();
        let columns = self.data_frame.width() - 1;
        ui.style_mut().wrap_mode = if self.settings.truncate {
            Some(TextWrapMode::Truncate)
        } else {
            Some(TextWrapMode::Extend)
        };
        TableBuilder::new(ui)
            .id_salt(id_salt)
            .striped(true)
            .resizable(true)
            .columns(Column::auto(), columns)
            .header(height + 2.0 * MARGIN.y, |mut row| {
                row.col(|ui| {
                    ui.heading(HASH);
                });
                for name in self
                    .data_frame
                    .get_column_names_str()
                    .into_iter()
                    .filter(|&name| !matches!(name, FATTY_ACID | THRESHOLD))
                {
                    row.col(|ui| {
                        ui.heading(name);
                    });
                }
            })
            .body(|mut body| {
                body.ui_mut().style_mut().wrap_mode = Some(TextWrapMode::Extend);
                body.rows(height, rows, |mut row| {
                    let index = row.index();
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
        match column {
            0 => {
                ui.label(row.to_string());
            }
            1 => {
                ui.label(self.data_frame[0].get(row)?.str_value());
            }
            column => {
                let mean_series = self.data_frame[column].struct_()?.field_by_name(MEAN)?;
                let mean = mean_series.f64()?.get(row);
                let standard_deviation_series = self.data_frame[column]
                    .struct_()?
                    .field_by_name(STANDARD_DEVIATION)?;
                let standard_deviation = standard_deviation_series.f64()?.get(row);
                let text = match mean {
                    Some(mean)
                        if self.settings.standard_deviation
                            && let Some(standard_deviation) = standard_deviation =>
                    {
                        WidgetText::from(format!("{mean} {standard_deviation}"))
                    }
                    Some(mean) => WidgetText::from(mean.to_string()),
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
            }
        }
        Ok(())
    }
}
