use super::{super::MARGIN, ID_SOURCE, Settings, State};
use crate::{
    app::{
        computers::{CalculationComputed, CalculationKey},
        widgets::{FattyAcidWidget, FloatWidget},
    },
    utils::{AnyValueExt as _, LayoutJobExt},
};
use egui::{
    Context, Frame, Id, Margin, Response, TextFormat, TextStyle, TextWrapMode, Ui, WidgetText,
    text::LayoutJob,
};
use egui_phosphor::regular::{HASH, MINUS, PLUS};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::DataFrameExt as _;
use std::ops::Range;
use tracing::instrument;

const INDEX: Range<usize> = 0..1;
const TAG: Range<usize> = INDEX.end..INDEX.end + 3;
const VALUE: Range<usize> = TAG.end..TAG.end + 1;
const LEN: usize = VALUE.end;
const TOP: &[Range<usize>] = &[INDEX, TAG, VALUE];

/// Table view
pub(super) struct TableView<'a> {
    source: &'a mut DataFrame,
    target: DataFrame,
    settings: &'a Settings,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(
        data_frame: &'a mut DataFrame,
        settings: &'a Settings,
        state: &'a mut State,
    ) -> Self {
        Self {
            source: data_frame,
            target: DataFrame::empty(),
            settings,
            state,
        }
    }
}

impl TableView<'_> {
    pub(super) fn show(&mut self, ui: &mut Ui) {
        self.target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    data_frame: self.source,
                    settings: self.settings,
                })
        });
        let id_salt = Id::new(ID_SOURCE).with("Table");
        if self.state.reset_table_state {
            let id = TableState::id(ui, Id::new(id_salt));
            TableState::reset(ui.ctx(), id);
            self.state.reset_table_state = false;
        }
        let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
        let num_rows = self.source.height() as u64 + 1;
        let num_columns = LEN;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default().resizable(self.settings.resizable);
                num_columns
            ])
            .num_sticky_cols(self.settings.sticky)
            .headers([
                HeaderRow {
                    height,
                    groups: TOP.to_vec(),
                },
                HeaderRow::new(height),
            ])
            .show(ui, self);
        if self.state.add_table_row {
            self.source.add_row().unwrap();
            self.state.add_table_row = false;
        }
        if let Some(index) = self.state.delete_table_row {
            self.source.delete_row(index).unwrap();
            self.state.delete_table_row = None;
        }
    }

    fn header_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        if self.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, INDEX) => {
                ui.heading(HASH);
            }
            (0, TAG) => {
                // CIRCLES_THREE
                ui.heading("TAG");
            }
            (0, VALUE) => {
                ui.heading("Value");
            }
            // Bottom
            (1, tag::SN1) => {
                ui.label(LayoutJob::subscripted_text(
                    ui,
                    "SN_1",
                    Some(TextStyle::Heading),
                    None,
                ));
            }
            (1, tag::SN2) => {
                ui.label(LayoutJob::subscripted_text(
                    ui,
                    "SN_2",
                    Some(TextStyle::Heading),
                    None,
                ));
            }
            (1, tag::SN3) => {
                ui.label(LayoutJob::subscripted_text(
                    ui,
                    "SN_3",
                    Some(TextStyle::Heading),
                    None,
                ));
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
        if !self.source.is_empty() {
            if row < self.source.height() {
                self.body_cell_content_ui(ui, row, column)?;
            } else {
                self.footer_cell_content_ui(ui, column)?;
            }
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
                if self.settings.editable {
                    if ui.button(MINUS).clicked() {
                        self.state.delete_table_row = Some(row);
                    }
                }
                ui.label(row.to_string());
            }
            (row, &tag::SN1) => {
                let stereospecific_number = self.target["Triacylglycerol"]
                    .struct_()?
                    .field_by_name("StereospecificNumber2")?;
                let fatty_acid = stereospecific_number
                    .struct_()?
                    .field_by_name("FattyAcid")?
                    .try_fatty_acid_list()?
                    .get(row);
                let label = stereospecific_number.struct_()?.field_by_name("Label")?;
                FattyAcidWidget::new(fatty_acid)
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text(label.str_value(row)?);
            }
            (row, &tag::SN2) => {
                let stereospecific_number = self.target["Triacylglycerol"]
                    .struct_()?
                    .field_by_name("StereospecificNumber2")?;
                let fatty_acid = stereospecific_number
                    .struct_()?
                    .field_by_name("FattyAcid")?
                    .try_fatty_acid_list()?
                    .get(row);
                let label = stereospecific_number.struct_()?.field_by_name("Label")?;
                FattyAcidWidget::new(fatty_acid)
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text(label.str_value(row)?);
            }
            (row, &tag::SN3) => {
                let stereospecific_number = self.target["Triacylglycerol"]
                    .struct_()?
                    .field_by_name("StereospecificNumber3")?;
                let fatty_acid = stereospecific_number
                    .struct_()?
                    .field_by_name("FattyAcid")?
                    .try_fatty_acid_list()?
                    .get(row);
                let label = stereospecific_number.struct_()?.field_by_name("Label")?;
                FattyAcidWidget::new(fatty_acid)
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text(label.str_value(row)?);
            }
            (row, &VALUE) => {
                let value = self.target["Value"].f64()?.get(row);
                FloatWidget::new(value)
                    .precision(Some(self.settings.precision))
                    .hover()
                    .show(ui);
            }
            _ => {}
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: Range<usize>) -> PolarsResult<()> {
        match column {
            INDEX => {
                if self.settings.editable {
                    if ui.button(PLUS).clicked() {
                        self.state.add_table_row = true;
                    }
                }
            }
            // experimental::SN123 => {
            //     FloatWidget::new(self.source["StereospecificNumber123"].f64()?.sum())
            //         .precision(Some(self.settings.precision))
            //         .hover()
            //         .show(ui)
            //         .response
            //         .on_hover_text("∑TAG");
            // }
            // experimental::SN2 => {
            //     FloatWidget::new(self.source["StereospecificNumber2"].f64()?.sum())
            //         .precision(Some(self.settings.precision))
            //         .hover()
            //         .show(ui)
            //         .response
            //         .on_hover_text("∑MAG");
            // }
            // calculated::sn123::D | calculated::sn2::D => {
            //     let name = match column {
            //         calculated::sn123::D => "StereospecificNumber123",
            //         calculated::sn2::D => "StereospecificNumber2",
            //         _ => unreachable!(),
            //     };
            //     FloatWidget::new(
            //         self.target[name]
            //             .struct_()?
            //             .field_by_name("Meta")?
            //             .struct_()?
            //             .field_by_name("Sum")?
            //             .f64()?
            //             .first(),
            //     )
            //     .precision(Some(self.settings.precision))
            //     .hover()
            //     .show(ui)
            //     .response
            //     .on_hover_text("∑D");
            // }
            // calculated::sn123::E => {
            //     FloatWidget::new(
            //         self.target["StereospecificNumber123"]
            //             .struct_()?
            //             .field_by_name("Data")?
            //             .struct_()?
            //             .field_by_name("E")?
            //             .f64()?
            //             .sum()
            //             .map(|e| 50.0 - e),
            //     )
            //     .precision(Some(self.settings.precision))
            //     .hover()
            //     .show(ui)
            //     .response
            //     .on_hover_text("50 - ∑E");
            // }
            // calculated::sn2::E => {
            //     FloatWidget::new(
            //         self.target["StereospecificNumber2"]
            //             .struct_()?
            //             .field_by_name("Data")?
            //             .struct_()?
            //             .field_by_name("E")?
            //             .f64()?
            //             .sum()
            //             .map(|e| 50.0 - e),
            //     )
            //     .precision(Some(self.settings.precision))
            //     .hover()
            //     .show(ui)
            //     .response
            //     .on_hover_text("50 - ∑E");
            // }
            // calculated::F => {
            //     FloatWidget::new(self.target["F"].f64()?.sum().map(|f| 100.0 - f))
            //         .precision(Some(self.settings.precision))
            //         .hover()
            //         .show(ui)
            //         .response
            //         .on_hover_text("100 - ∑F");
            // }
            _ => {} // _ => unreachable!(),
        }
        Ok(())
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
        if cell.row_nr % 2 == 0 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
        Frame::new()
            .inner_margin(Margin::from(MARGIN))
            .show(ui, |ui| {
                self.cell_content_ui(ui, cell.row_nr as _, cell.col_nr..cell.col_nr + 1)
                    .unwrap()
            });
    }

    fn row_top_offset(&self, ctx: &Context, _table_id: Id, row: u64) -> f32 {
        row as f32 * (ctx.style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}

mod tag {
    use super::*;

    pub(super) const SN1: Range<usize> = TAG.start..TAG.start + 1;
    pub(super) const SN2: Range<usize> = SN1.end..SN1.end + 1;
    pub(super) const SN3: Range<usize> = SN2.end..SN2.end + 1;
}
