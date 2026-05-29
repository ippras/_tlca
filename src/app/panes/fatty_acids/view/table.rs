use crate::{
    app::{
        panes::MARGIN,
        states::fatty_acids::{ID_SOURCE, settings::Settings},
    },
    r#const::MAJOR,
};
use egui::{Context, Frame, Id, Label, Margin, TextStyle, TextWrapMode, Ui, Widget};
use egui_l20n::prelude::*;
use egui_phosphor::regular::HASH;
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use fatty_acid_names_l10n::egui::Names;
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::ops::Range;
use tracing::instrument;
use widgets::polars::array::Float64Array;

const NUM_COLUMNS: usize = top::FATTY_ACID.end;

/// Table view
pub(crate) struct TableView<'a> {
    data_frame: &'a DataFrame,
    settings: &'a mut Settings,
}

impl<'a> TableView<'a> {
    pub(crate) fn new(data_frame: &'a DataFrame, settings: &'a mut Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }
}

impl TableView<'_> {
    #[instrument(skip(self, ui), err)]
    pub(crate) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Table");
        if self.settings.reset {
            let id = TableState::id(ui, Id::new(id_salt));
            TableState::reset(ui.ctx(), id);
            self.settings.reset = false;
        }
        let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
        // println!("self.data_frame: {:?}", self.data_frame);
        let num_rows = self.data_frame.height() as u64;
        let value = self.data_frame.width() - 3;
        let num_columns = NUM_COLUMNS + value;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default().resizable(self.settings.resizable);
                num_columns
            ])
            .num_sticky_cols(self.settings.sticky_columns)
            .headers([
                HeaderRow {
                    height,
                    groups: vec![
                        top::INDEX,
                        top::LABEL,
                        top::FATTY_ACID,
                        NUM_COLUMNS..num_columns,
                    ],
                },
                HeaderRow::new(height),
            ])
            .show(ui, self);
        Ok(())
    }

    fn header(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        if self.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, top::INDEX) => {
                ui.heading(HASH);
            }
            (0, top::LABEL) => {
                ui.heading(ui.localize(LABEL));
            }
            (0, top::FATTY_ACID) => {
                ui.heading(ui.localize(FATTY_ACID));
            }
            (0, _) => {
                ui.heading(ui.localize("Value"));
            }
            // Bottom
            (1, top::INDEX | top::LABEL | top::FATTY_ACID) => {}
            (1, column) => {
                ui.heading(self.data_frame[column.start - 1].name().to_string());
            }
            _ => {}
        };
    }

    fn body(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) -> PolarsResult<()> {
        if let Some(filter) = self.data_frame[MAJOR].bool()?.get(row)
            && !filter
        {
            ui.multiply_opacity(ui.visuals().disabled_alpha());
        }
        match (row, column) {
            (row, top::INDEX) => {
                ui.label(row.to_string());
            }
            (row, top::LABEL) => {
                let text = self.data_frame[LABEL].str()?.get(row).display().to_string();
                let mut label = Label::new(text);
                if self.settings.truncate {
                    label = label.truncate();
                }
                label.ui(ui);
            }
            (row, top::FATTY_ACID) => {
                let id = self
                    .data_frame
                    .try_fatty_acid()?
                    .id()?
                    .get(row)
                    .display()
                    .to_string();
                let text = self
                    .data_frame
                    .try_fatty_acid()?
                    .delta()?
                    .get(row)
                    .display()
                    .to_string();
                let mut label = Label::new(text);
                if self.settings.truncate {
                    label = label.truncate();
                }
                let response = label.ui(ui);
                response.on_hover_ui(|ui| {
                    Names::builder().id(&id).build().ui(ui);
                });
            }
            (row, column) => {
                Float64Array::builder()
                    .series(&self.data_frame[column.start - 1].as_materialized_series())
                    .row(row)
                    .mean(self.settings.mean.mean)
                    .standard_deviation(self.settings.mean.standard_deviation)
                    .relative(self.settings.mean.kind.is_relative())
                    .build()
                    .show(ui)?;
            }
        }
        Ok(())
    }
}

impl TableDelegate for TableView<'_> {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        Frame::new()
            .inner_margin(Margin::from(MARGIN))
            .show(ui, |ui| {
                self.header(ui, cell.row_nr, cell.col_range.clone())
            });
    }

    fn cell_ui(&mut self, ui: &mut Ui, cell: &CellInfo) {
        if cell.row_nr.is_multiple_of(2) {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
        Frame::new()
            .inner_margin(Margin::from(MARGIN))
            .show(ui, |ui| {
                _ = self.body(ui, cell.row_nr as _, cell.col_nr..cell.col_nr + 1);
            });
    }

    fn row_top_offset(&self, ctx: &Context, _table_id: Id, row: u64) -> f32 {
        row as f32 * (ctx.global_style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}

mod top {
    use super::*;

    pub(super) const INDEX: Range<usize> = 0..1;
    pub(super) const LABEL: Range<usize> = INDEX.end..INDEX.end + 1;
    pub(super) const FATTY_ACID: Range<usize> = LABEL.end..LABEL.end + 1;
    // pub(super) const ID: Range<usize> = INDEX.end..INDEX.end + 1;
}
