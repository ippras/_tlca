use crate::{
    app::{
        computers::fatty_acids::format::{Computed as FormatComputed, Key as FormatKey},
        panes::{MARGIN, mean_and_standard_deviation},
        states::fatty_acids::{ID_SOURCE, State},
    },
    utils::HashedDataFrame,
};
use egui::{Context, Frame, Id, Margin, Response, TextStyle, TextWrapMode, Ui, WidgetText};
use egui_l20n::UiExt;
use egui_phosphor::regular::HASH;
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use polars::prelude::*;
use std::ops::Range;
use tracing::instrument;

const INDEX: Range<usize> = 0..1;
const ID: Range<usize> = INDEX.end..INDEX.end + 1;
const LEN: usize = ID.end;

/// Table view
pub(super) struct TableView<'a> {
    frame: &'a HashedDataFrame,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(frame: &'a HashedDataFrame, state: &'a mut State) -> Self {
        Self { frame, state }
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
        let num_rows = self.frame.height() as u64 + 1;
        let value = self.frame.width() - 2;
        let num_columns = LEN + value;
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
                    groups: vec![INDEX, ID, LEN..num_columns],
                },
                HeaderRow::new(height),
            ])
            .show(ui, self);
        Ok(())
    }

    fn header_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        if self.state.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, INDEX) => {
                ui.heading(HASH);
            }
            (0, ID) => {
                ui.heading(ui.localize("Label"));
            }
            (0, _) => {
                ui.heading(ui.localize("Value"));
            }
            // Bottom
            (1, INDEX) => {}
            (1, ID) => {}
            (1, range) => {
                ui.heading(self.frame[range.start].name().to_string());
            }
            _ => {}
        };
    }

    #[instrument(skip(self, ui), err)]
    fn cell_content_ui(
        &mut self,
        ui: &mut Ui,
        row: usize,
        column: Range<usize>,
    ) -> PolarsResult<()> {
        if row < self.frame.height() {
            self.body_cell_content_ui(ui, row, column)?;
        } else {
            self.footer_cell_content_ui(ui, column)?;
        }
        Ok(())
    }

    fn body_cell_content_ui(
        &mut self,
        ui: &mut Ui,
        row: usize,
        column: Range<usize>,
    ) -> PolarsResult<()> {
        match (row, &column) {
            (row, &INDEX) => {
                ui.label(row.to_string());
            }
            (row, &ID) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<FormatComputed>().get(FormatKey::new(
                        &self.frame,
                        ID.start,
                        &self.state.settings,
                    ))
                });
                let label = data_frame[LABEL].get(row)?.str_value();
                let response = ui.label(label);
                if response.hovered()
                    && let Some(fatty_acid) = data_frame[FATTY_ACID].str()?.get(row)
                {
                    response.on_hover_ui(|ui| {
                        ui.set_max_width(ui.spacing().tooltip_width);
                        ui.label(fatty_acid);
                    });
                }
            }
            (row, range) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<FormatComputed>().get(FormatKey::new(
                        &self.frame,
                        range.start,
                        &self.state.settings,
                    ))
                });
                self.mean_and_standard_deviation(ui, range.start, row)?;
            }
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: Range<usize>) -> PolarsResult<()> {
        match column {
            INDEX | ID => {}
            range => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<FormatComputed>().get(FormatKey::new(
                        &self.frame,
                        range.start,
                        &self.state.settings,
                    ))
                });
                let row = data_frame.height() - 1;
                mean_and_standard_deviation(ui, &data_frame, row)?;
            }
        }
        Ok(())
    }

    fn mean_and_standard_deviation(
        &self,
        ui: &mut Ui,
        column: usize,
        row: usize,
    ) -> PolarsResult<Response> {
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<FormatComputed>().get(FormatKey::new(
                &self.frame,
                column,
                &self.state.settings,
            ))
        });
        let mean = data_frame["Mean"].str()?.get(row);
        let standard_deviation = data_frame["StandardDeviation"].str()?.get(row);
        let text = match mean {
            Some(mean)
                if self.state.settings.standard_deviation
                    && let Some(standard_deviation) = standard_deviation =>
            {
                WidgetText::from(format!("{mean} {standard_deviation}"))
            }
            Some(mean) => WidgetText::from(mean.to_string()),
            None => WidgetText::from("—"),
        };
        let mut response = ui.label(text);
        if response.hovered() {
            // Standard deviation
            if let Some(text) = standard_deviation {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize("StandardDeviation"));
                    ui.label(text);
                });
            }
        }
        Ok(response)
    }
}

impl TableDelegate for TableView<'_> {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        Frame::new()
            .inner_margin(Margin::from(MARGIN))
            .show(ui, |ui| {
                self.header_cell_content_ui(ui, cell.row_nr, cell.col_range.clone())
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
        row as f32 * (ctx.style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}
