use super::{super::MARGIN, ID_SOURCE, Settings, State, settings::Kind};
use crate::{
    app::{
        computers::{CalculationComputed, CalculationKey},
        widgets::{FattyAcidWidget, FloatWidget},
    },
    utils::{AnyValueExt as _, Hashed, LayoutJobExt},
};
use egui::{
    Context, Frame, Id, Margin, Response, TextFormat, TextStyle, TextWrapMode, Ui, WidgetText,
    text::LayoutJob, util::hash,
};
use egui_phosphor::regular::{HASH, MINUS, PLUS};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use metadata::MetaDataFrame;
use polars::prelude::*;
use polars_ext::prelude::DataFrameExt as _;
use std::ops::Range;
use tracing::instrument;

const INDEX: Range<usize> = 0..1;
const TAG: Range<usize> = INDEX.end..INDEX.end + 3;
const LEN: usize = TAG.end;

/// Table view
pub(super) struct TableView<'a> {
    source: &'a mut [Hashed<MetaDataFrame>],
    target: DataFrame,
    settings: &'a Settings,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(
        frames: &'a mut [Hashed<MetaDataFrame>],
        settings: &'a Settings,
        state: &'a mut State,
    ) -> Self {
        Self {
            source: frames,
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
                    frames: self.source,
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
        let num_rows = self.target.height() as u64 + 1;
        let value = self.target.width() - 1;
        let num_columns = LEN + value;
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
                    groups: vec![INDEX, TAG, LEN..num_columns],
                },
                HeaderRow::new(height),
            ])
            .show(ui, self);
        if self.state.add_table_row {
            self.source[0].value.data.add_row().unwrap();
            self.source[0].hash = hash(&self.source[0].value);
            self.state.add_table_row = false;
        }
        if let Some(index) = self.state.delete_table_row {
            self.source[0].value.data.delete_row(index).unwrap();
            self.source[0].hash = hash(&self.source[0].value);
            self.state.delete_table_row = None;
        }
        if let Some(index) = self.state.take_firts_table_rows {
            self.source[0].value.data.firts_rows_to(index);
            self.source[0].hash = hash(&self.source[0].value);
            self.state.add_table_row = false;
        }
        if let Some(index) = self.state.take_last_table_rows {
            self.source[0].value.data.last_rows_from(index);
            self.source[0].hash = hash(&self.source[0].value);
            self.state.add_table_row = false;
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
            (0, _) => {
                ui.heading("Value");
            }
            // Bottom
            (1, INDEX) => {}
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
            (1, range) => {
                ui.heading(self.target[range.start - LEN + 1].name().to_string());
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
            if row < self.target.height() {
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
                let response = ui.label(row.to_string());
                if self.settings.editable {
                    response.context_menu(|ui| {
                        if ui.button("Firts rows to").clicked() {
                            self.state.take_firts_table_rows = Some(row);
                        }
                        if ui.button("Last rows from").clicked() {
                            self.state.take_last_table_rows = Some(row);
                        }
                    });
                }
            }
            (row, &tag::SN1) => {
                let stereospecific_number = self.target["Triacylglycerol"]
                    .struct_()?
                    .field_by_name("StereospecificNumber1")?;
                let fatty_acid = stereospecific_number
                    .struct_()?
                    .field_by_name("FattyAcid")?
                    .try_fatty_acid_list()?
                    .get(row);
                let label = stereospecific_number.struct_()?.field_by_name("Label")?;
                let mut inner_response = FattyAcidWidget::new(fatty_acid)
                    .editable(self.settings.editable && self.source.len() == 1)
                    .hover()
                    .show(ui);
                inner_response.response =
                    inner_response.response.on_hover_text(label.str_value(row)?);
                // if inner_response.response.changed() {
                //     self.source[0]
                //         .value
                //         .data
                //         .try_apply("Triacylglycerol", |series| {
                //             let stereospecific_number =
                //                 series.struct_()?.field_by_name("StereospecificNumber1")?;
                //             let fatty_acid = stereospecific_number
                //                 .struct_()?
                //                 .field_by_name("FattyAcid")?;
                //             let l = fatty_acid.fatty_acid_list().into_list().set();
                //             Ok(fatty_acid)
                //         })?;
                //     stereospecific_number
                //         .struct_()?
                //         .try_apply_fields(|s| s)
                //         .field_by_name("FattyAcid")?
                //         .apply(|s| s);
                //     // fatty_acid.apply
                //     self.source[0].hash = hash(&self.source[0].value);
                // }
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
                    .editable(self.settings.editable && self.source.len() == 1)
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
                    .editable(self.settings.editable && self.source.len() == 1)
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text(label.str_value(row)?);
            }
            (row, range) => {
                let value = self.target[range.start - LEN + 1].f64()?.get(row);
                FloatWidget::new(value)
                    .editable(self.settings.editable)
                    .percent(self.settings.percent)
                    .precision(Some(self.settings.precision))
                    .hover()
                    .show(ui);
            }
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
            tag::SN1 => {}
            tag::SN2 => {}
            tag::SN3 => {}
            range => {}
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

// fn update_triacylglycerol(
//     row: usize,
//     value: Option<FattyAcidChunked>,
// ) -> impl FnMut(&Series) -> PolarsResult<Series> + 'static {
//     move |series| {
//         let out = series
//             .fatty_acid_list()
//             .iter()
//             .enumerate()
//             .map(|(index, fatty_acid)| {
//                 Ok(if index == row {
//                     println!("value: {value:?}");
//                     match value.clone() {
//                         Some(value) => Some(value.into_struct(PlSmallStr::EMPTY)?.into_series()),
//                         None => None,
//                     }
//                 } else {
//                     Some(fatty_acid.into_struct(PlSmallStr::EMPTY)?.into_series())
//                 })
//             })
//             .collect::<PolarsResult<ListChunked>>()?;
//         Ok(out.into_series())
//     }
// }

mod tag {
    use super::*;

    pub(super) const SN1: Range<usize> = TAG.start..TAG.start + 1;
    pub(super) const SN2: Range<usize> = SN1.end..SN1.end + 1;
    pub(super) const SN3: Range<usize> = SN2.end..SN2.end + 1;
}
