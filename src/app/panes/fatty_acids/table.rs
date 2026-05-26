use crate::{
    app::{
        panes::MARGIN,
        states::fatty_acids::{ID_SOURCE, State},
        widgets::mean_and_standard_deviation::MeanAndStandardDeviation,
    },
    r#const::THRESHOLD,
};
use egui::{Context, Frame, Id, Margin, TextStyle, TextWrapMode, Ui};
use egui_l20n::prelude::*;
use egui_phosphor::regular::HASH;
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use polars::prelude::*;
use std::ops::Range;
use tracing::instrument;

const NUM_COLUMNS: usize = top::ID.end;

/// Table view
pub(super) struct TableView<'a> {
    data_frame: &'a DataFrame,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, state: &'a mut State) -> Self {
        Self { data_frame, state }
    }
}

impl TableView<'_> {
    #[instrument(skip(self, ui), err)]
    pub(super) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Table");
        if self.state.reset_table_state {
            let id = TableState::id(ui, Id::new(id_salt));
            TableState::reset(ui.ctx(), id);
            self.state.reset_table_state = false;
        }
        let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
        let num_rows = self.data_frame.height() as u64;
        let value = self.data_frame.width() - 3;
        let num_columns = NUM_COLUMNS + value;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default()
                    .resizable(self.state.settings.resizable);
                num_columns
            ])
            .num_sticky_cols(self.state.settings.sticky)
            .headers([
                HeaderRow {
                    height,
                    groups: vec![top::INDEX, top::ID, NUM_COLUMNS..num_columns],
                },
                HeaderRow::new(height),
            ])
            .show(ui, self);
        Ok(())
    }

    fn header(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        if self.state.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, top::INDEX) => {
                ui.heading(HASH);
            }
            (0, top::ID) => {
                ui.heading(ui.localize("Label"));
            }
            (0, _) => {
                ui.heading(ui.localize("Value"));
            }
            // Bottom
            (1, top::INDEX) => {}
            (1, top::ID) => {}
            (1, column) => {
                ui.heading(self.data_frame[column.start].name().to_string());
            }
            _ => {}
        };
    }

    fn cell_content_ui(
        &mut self,
        ui: &mut Ui,
        row: usize,
        column: Range<usize>,
    ) -> PolarsResult<()> {
        if let Some(threshold) = self.data_frame[THRESHOLD].bool()?.get(row)
            && !threshold
        {
            ui.multiply_opacity(ui.visuals().disabled_alpha());
        }
        match (row, column) {
            (row, top::INDEX) if row + 1 < self.data_frame.height() => {
                ui.label(row.to_string());
            }
            (row, top::ID) => {
                if let Some(label) = self.data_frame[LABEL].str()?.get(row) {
                    let response = ui.label(label);
                    if response.hovered()
                        && let Some(fatty_acid) = self.data_frame[FATTY_ACID].str()?.get(row)
                    {
                        response.on_hover_ui(|ui| {
                            ui.set_max_width(ui.spacing().tooltip_width);
                            ui.label(fatty_acid);
                        });
                    }
                }
            }
            (row, column) => {
                MeanAndStandardDeviation::new(&self.data_frame, column.start, row)
                    .with_standard_deviation(self.state.settings.standard_deviation)
                    .with_sample(true)
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
                _ = self.cell_content_ui(ui, cell.row_nr as _, cell.col_nr..cell.col_nr + 1);
            });
    }

    fn row_top_offset(&self, ctx: &Context, _table_id: Id, row: u64) -> f32 {
        row as f32 * (ctx.global_style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}

mod top {
    use super::*;

    pub(super) const INDEX: Range<usize> = 0..1;
    pub(super) const ID: Range<usize> = INDEX.end..INDEX.end + 1;
}
