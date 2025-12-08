use crate::{
    app::{
        panes::MARGIN,
        states::fatty_acids::{ID_SOURCE, Settings},
    },
    r#const::MEAN,
};
use egui::{Id, TextStyle, TextWrapMode, Ui};
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use tracing::instrument;

/// Indices widget
pub(crate) struct Indices<'a> {
    pub data_frame: &'a DataFrame,
    pub settings: &'a Settings,
}

impl<'a> Indices<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }

    #[instrument(skip_all, err)]
    pub(crate) fn show(mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Indices");
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
                ui.label(self.data_frame[0].get(row)?.str_value());
            }
            column => {
                if let Some(mean) = self.data_frame[column]
                    .struct_()?
                    .field_by_name(MEAN)?
                    .f64()?
                    .get(row)
                {
                    let text = format!("{mean:.0$}", self.settings.precision);
                    ui.label(text);
                    // Label::new(RichText::new(text).color(color))
                    //     .ui(ui)
                    //     .on_hover_text(value.to_string())
                    //     .on_hover_text(format!("{sign:?}"));
                }
            }
        }
        Ok(())
    }
}
