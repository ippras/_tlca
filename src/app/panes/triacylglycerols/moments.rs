use crate::app::{
    panes::MARGIN,
    states::triacylglycerols::{ID_SOURCE, settings::Settings},
};
#[cfg(feature = "markdown")]
use crate::r#const::markdown::{KURTOSIS, SKEWNESS};
use egui::{Id, TextStyle, TextWrapMode, Ui};
#[cfg(feature = "markdown")]
use egui_ext::Markdown;
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use tracing::instrument;

/// Moments
pub struct Moments<'a> {
    pub data_frame: &'a DataFrame,
    pub settings: &'a Settings,
}

impl<'a> Moments<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }
}

impl Moments<'_> {
    #[instrument(skip_all, err)]
    pub fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Moments");
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
            .columns(Column::auto(), columns)
            .header(height + 2.0 * MARGIN.y, |mut row| {
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
                if let Some(name) = self.data_frame["Moment"].str()?.get(row) {
                    #[allow(unused_variables)]
                    let response = ui.label(name);
                    #[cfg(feature = "markdown")]
                    response.on_hover_ui(|ui| match name {
                        "Kurtosis" => ui.markdown(KURTOSIS),
                        "Skewness" => ui.markdown(SKEWNESS),
                        _ => {}
                    });
                }
            }
            column => {
                if let Some(value) = self.data_frame[column].f64()?.get(row) {
                    let text = format!("{value:.0$}", self.settings.precision);
                    ui.label(text).on_hover_text(value.to_string());
                }
            }
        }
        Ok(())
    }
}
