use super::{super::MARGIN, ID_SOURCE, Settings, State};
use crate::{
    app::{
        computers::{CalculationComputed, CalculationKey},
        widgets::{FattyAcidWidget, FloatWidget},
    },
    utils::{AnyValueExt as _, UiExt as _},
};
use egui::{Context, Frame, Id, Margin, Response, TextStyle, TextWrapMode, Ui};
use egui_phosphor::regular::{MINUS, PLUS};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::DataFrameExt as _;
use std::ops::Range;
use tracing::instrument;

const ID: Range<usize> = 0..2;
const EXPERIMENTAL: Range<usize> = ID.end..ID.end + 2;
const CALCULATED: Range<usize> = EXPERIMENTAL.end..EXPERIMENTAL.end + 11;
const LEN: usize = CALCULATED.end;
const TOP: &[Range<usize>] = &[ID, EXPERIMENTAL, CALCULATED];
const MIDDLE: &[Range<usize>] = &[
    id::INDEX,
    id::FA,
    experimental::SN123,
    experimental::SN2,
    calculated::SN123,
    calculated::SN2,
    calculated::F,
];

const A: &str = "$A \\in [MIN, MAX]$";
const C: &str = "$C = |A - B| / A$";
const E: &str = "$E = 50 * C * D / ∑ D$";

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
                HeaderRow {
                    height,
                    groups: MIDDLE.to_vec(),
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
            (0, ID) => {
                ui.heading("ID");
            }
            (0, EXPERIMENTAL) => {
                ui.heading("Experimental");
            }
            (0, CALCULATED) => {
                ui.heading("Calculated");
            }
            // Middle
            (1, id::INDEX) => {
                ui.heading("Index");
            }
            (1, id::FA) => {
                ui.heading("FA");
            }
            (1, experimental::SN123 | calculated::SN123) => {
                ui.heading("SN123");
            }
            (1, experimental::SN2 | calculated::SN2) => {
                ui.heading("SN2");
            }
            // Bottom
            (2, calculated::sn123::A | calculated::sn2::A) => {
                ui.heading("A").on_hover_ui(|ui| {
                    ui.markdown_ui(A);
                });
            }
            (2, calculated::sn123::B | calculated::sn2::B) => {
                ui.heading("B");
            }
            (2, calculated::sn123::C | calculated::sn2::C) => {
                ui.heading("C").on_hover_ui(|ui| {
                    ui.markdown_ui(C);
                });
            }
            (2, calculated::sn123::D | calculated::sn2::D) => {
                ui.heading("D");
            }
            (2, calculated::sn123::E | calculated::sn2::E) => {
                ui.heading("E").on_hover_ui(|ui| {
                    ui.markdown_ui(E);
                });
            }
            (2, calculated::F) => {
                ui.heading("F");
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
            (row, &id::INDEX) => {
                if self.settings.editable {
                    if ui.button(MINUS).clicked() {
                        self.state.delete_table_row = Some(row);
                    }
                }
                ui.label(row.to_string());
            }
            (row, &id::FA) => {
                let inner_response =
                    FattyAcidWidget::new(self.source.try_fatty_acid_list()?.get(row))
                        .editable(self.settings.editable)
                        .hover()
                        .show(ui);
                if inner_response.response.changed() {
                    self.source
                        .try_apply("FattyAcid", update_fatty_acid(row, inner_response.inner))?;
                }
            }
            (row, &experimental::SN123) => {
                self.rw(ui, row, "StereospecificNumber123")?;
            }
            (row, &experimental::SN2) => {
                self.rw(ui, row, "StereospecificNumber2")?;
            }
            (row, &calculated::sn123::A | &calculated::sn2::A) => {
                let name = match column {
                    calculated::sn123::A => "StereospecificNumber123",
                    calculated::sn2::A => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                self.ro(
                    ui,
                    self.target[name]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("A")?
                        .f64()?
                        .get(row),
                )?
                .on_hover_ui(|ui| {
                    ui.horizontal(|ui| -> PolarsResult<()> {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("[");
                        FloatWidget::new(
                            self.target[name]
                                .struct_()?
                                .field_by_name("Meta")?
                                .struct_()?
                                .field_by_name("Min")?
                                .f64()?
                                .get(row),
                        )
                        .show(ui);
                        ui.label(",");
                        FloatWidget::new(
                            self.target[name]
                                .struct_()?
                                .field_by_name("Meta")?
                                .struct_()?
                                .field_by_name("Max")?
                                .f64()?
                                .get(row),
                        )
                        .show(ui);
                        ui.label("]");
                        Ok(())
                    });
                });
            }
            (row, &calculated::sn123::B | &calculated::sn2::B) => {
                let name = match column {
                    calculated::sn123::B => "StereospecificNumber123",
                    calculated::sn2::B => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                self.ro(
                    ui,
                    self.target[name]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("B")?
                        .f64()?
                        .get(row),
                )?;
            }
            (row, &calculated::sn123::C | &calculated::sn2::C) => {
                let name = match column {
                    calculated::sn123::C => "StereospecificNumber123",
                    calculated::sn2::C => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                let a = self.target[name]
                    .struct_()?
                    .field_by_name("Data")?
                    .struct_()?
                    .field_by_name("A")?
                    .get(row)?
                    .display();
                let b = self.target[name]
                    .struct_()?
                    .field_by_name("Data")?
                    .struct_()?
                    .field_by_name("B")?
                    .get(row)?
                    .display();
                self.ro(
                    ui,
                    self.target[name]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("C")?
                        .f64()?
                        .get(row),
                )?
                .on_hover_ui(|ui| {
                    ui.markdown_ui(&format!("|{a} - {b}| / {a}"));
                });
            }
            (row, &calculated::sn123::D | &calculated::sn2::D) => {
                let name = match column {
                    calculated::sn123::D => "StereospecificNumber123",
                    calculated::sn2::D => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                let d = self.target[name]
                    .struct_()?
                    .field_by_name("Data")?
                    .struct_()?
                    .field_by_name("D")?
                    .f64()?
                    .get(row);
                let response = self.ro(ui, d)?;
                if let Ok((Some(d), Some(sum))) = (|| -> PolarsResult<_> {
                    let sum = self.target[name]
                        .struct_()?
                        .field_by_name("Meta")?
                        .struct_()?
                        .field_by_name("Sum")?
                        .f64()?
                        .get(row);
                    Ok((d, sum))
                })() {
                    response.on_hover_ui(|ui| {
                        ui.label(format!(
                            "{d} / {} = {}",
                            AnyValue::Float64(sum),
                            AnyValue::Float64(d / sum),
                        ));
                    });
                }
            }
            (row, &calculated::sn123::E | &calculated::sn2::E) => {
                let name = match column {
                    calculated::sn123::E => "StereospecificNumber123",
                    calculated::sn2::E => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                let c = self.target[name]
                    .struct_()?
                    .field_by_name("Data")?
                    .struct_()?
                    .field_by_name("C")?
                    .get(row)?
                    .display();
                let d = self.target[name]
                    .struct_()?
                    .field_by_name("Data")?
                    .struct_()?
                    .field_by_name("D")?
                    .get(row)?
                    .display();
                let sum = self.target[name]
                    .struct_()?
                    .field_by_name("Meta")?
                    .struct_()?
                    .field_by_name("Sum")?
                    .get(row)?
                    .display();
                self.ro(
                    ui,
                    self.target[name]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .get(row),
                )?
                .on_hover_ui(|ui| {
                    ui.markdown_ui(&format!("50 * {c} * {d} / {sum}"));
                });
            }
            (row, &calculated::F) => {
                self.ro(ui, self.target["F"].f64()?.get(row))?;
            }
            _ => {}
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: Range<usize>) -> PolarsResult<()> {
        match column {
            id::INDEX => {
                if self.settings.editable {
                    if ui.button(PLUS).clicked() {
                        self.state.add_table_row = true;
                    }
                }
            }
            experimental::SN123 => {
                FloatWidget::new(self.source["StereospecificNumber123"].f64()?.sum())
                    .precision(Some(self.settings.precision))
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text("∑TAG");
            }
            experimental::SN2 => {
                FloatWidget::new(self.source["StereospecificNumber2"].f64()?.sum())
                    .precision(Some(self.settings.precision))
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text("∑MAG");
            }
            calculated::sn123::D | calculated::sn2::D => {
                let name = match column {
                    calculated::sn123::D => "StereospecificNumber123",
                    calculated::sn2::D => "StereospecificNumber2",
                    _ => unreachable!(),
                };
                FloatWidget::new(
                    self.target[name]
                        .struct_()?
                        .field_by_name("Meta")?
                        .struct_()?
                        .field_by_name("Sum")?
                        .f64()?
                        .first(),
                )
                .precision(Some(self.settings.precision))
                .hover()
                .show(ui)
                .response
                .on_hover_text("∑D");
            }
            calculated::sn123::E => {
                FloatWidget::new(
                    self.target["StereospecificNumber123"]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .sum()
                        .map(|e| 50.0 - e),
                )
                .precision(Some(self.settings.precision))
                .hover()
                .show(ui)
                .response
                .on_hover_text("50 - ∑E");
            }
            calculated::sn2::E => {
                FloatWidget::new(
                    self.target["StereospecificNumber2"]
                        .struct_()?
                        .field_by_name("Data")?
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .sum()
                        .map(|e| 50.0 - e),
                )
                .precision(Some(self.settings.precision))
                .hover()
                .show(ui)
                .response
                .on_hover_text("50 - ∑E");
            }
            calculated::F => {
                FloatWidget::new(self.target["F"].f64()?.sum().map(|f| 100.0 - f))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .show(ui)
                    .response
                    .on_hover_text("100 - ∑F");
            }
            _ => {} // _ => unreachable!(),
        }
        Ok(())
    }

    fn rw(&mut self, ui: &mut Ui, row: usize, column: &str) -> PolarsResult<Response> {
        let inner_response = FloatWidget::new(self.source[column].f64()?.get(row))
            .editable(self.settings.editable)
            .precision(Some(self.settings.precision))
            .hover()
            .show(ui);
        if inner_response.response.changed() {
            self.source
                .try_apply(column, update_float(row, inner_response.inner))?;
        }
        Ok(inner_response.response)
    }

    fn ro(&self, ui: &mut Ui, value: Option<f64>) -> PolarsResult<Response> {
        Ok(FloatWidget::new(value)
            .precision(Some(self.settings.precision))
            .hover()
            .show(ui)
            .response)
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

fn update_fatty_acid(
    row: usize,
    value: Option<FattyAcidChunked>,
) -> impl FnMut(&Series) -> PolarsResult<Series> + 'static {
    move |series| {
        let out = series
            .fatty_acid_list()
            .iter()
            .enumerate()
            .map(|(index, fatty_acid)| {
                Ok(if index == row {
                    println!("value: {value:?}");
                    match value.clone() {
                        Some(value) => Some(value.into_struct(PlSmallStr::EMPTY)?.into_series()),
                        None => None,
                    }
                } else {
                    Some(fatty_acid.into_struct(PlSmallStr::EMPTY)?.into_series())
                })
            })
            .collect::<PolarsResult<ListChunked>>()?;
        println!("out: {out:?}");
        Ok(out.into_series())
        // unimplemented!();
        // let list = series.fatty_acid_list().as_list();
        // println!("fatty_acid_series: {list:?}");
        // let mut builder = AnonymousOwnedListBuilder::new(
        //     list.name().clone(),
        //     list.len(),
        //     list.dtype().inner_dtype().cloned(),
        // );
        // // let value = value.map(|value| value.into_struct(PlSmallStr::EMPTY)?.into_series());
        // for index in 0..list.len() {
        //     if index == row {
        //         let value = match value.clone() {
        //             Some(value) => Some(value.into_struct(PlSmallStr::EMPTY)?.into_series()),
        //             None => None,
        //         };
        //         builder.append_opt_series(value.as_ref())?;
        //     } else {
        //         builder.append_opt_series(list.get_as_series(index).as_ref());
        //     }
        // }
        // Ok(builder
        //     .finish()
        //     // .cast(&DataType::List(FATTY_ACID_DATA_TYPE.clone().boxed()))?
        //     .into_series())

        // unimplemented!()
        // for index in 0..fatty_acid_series.len() {
        //     let mut fatty_acid = fatty_acid_series.get(index)?;
        //     if index == row {
        //         fatty_acid = value.clone();
        //     }
        //     let fatty_acid = fatty_acid.as_ref();
        //     // Carbons
        //     carbons.append_option(fatty_acid.map(|fatty_acid| fatty_acid.carbons));
        //     // Unsaturated
        //     if let Some(fatty_acid) = fatty_acid {
        //         let mut index = PrimitiveChunkedBuilder::<UInt8Type>::new(
        //             "Index".into(),
        //             fatty_acid.unsaturated.len(),
        //         );
        //         let mut isomerism = PrimitiveChunkedBuilder::<Int8Type>::new(
        //             "Isomerism".into(),
        //             fatty_acid.unsaturated.len(),
        //         );
        //         let mut unsaturation = PrimitiveChunkedBuilder::<UInt8Type>::new(
        //             "Unsaturation".into(),
        //             fatty_acid.unsaturated.len(),
        //         );
        //         for unsaturated in &fatty_acid.unsaturated {
        //             index.append_option(unsaturated.index);
        //             isomerism.append_option(unsaturated.isomerism.map(|isomerism| isomerism as _));
        //             unsaturation.append_option(
        //                 unsaturated
        //                     .unsaturation
        //                     .map(|unsaturation| unsaturation as _),
        //             );
        //         }
        //         unsaturated.append_series(
        //             &StructChunked::from_series(
        //                 PlSmallStr::EMPTY,
        //                 fatty_acid.unsaturated.len(),
        //                 [
        //                     index.finish().into_series(),
        //                     isomerism.finish().into_series(),
        //                     unsaturation.finish().into_series(),
        //                 ]
        //                 .iter(),
        //             )?
        //             .into_series(),
        //         )?;
        //     } else {
        //         unsaturated.append_opt_series(None)?;
        //     }
        // }
        // Ok(StructChunked::from_series(
        //     series.name().clone(),
        //     fatty_acid_series.len(),
        //     [
        //         carbons.finish().into_series(),
        //         unsaturated.finish().into_series(),
        //     ]
        //     .iter(),
        // )?
        // .into_series())
    }
}

fn update_float(row: usize, new: Option<f64>) -> impl FnMut(&Series) -> PolarsResult<Series> {
    move |series| {
        Ok(series
            .f64()?
            .iter()
            .enumerate()
            .map(|(index, mut value)| {
                if index == row {
                    value = new;
                }
                Ok(value)
            })
            .collect::<PolarsResult<Float64Chunked>>()?
            .into_series())
    }
}

mod id {
    use super::*;

    pub(super) const INDEX: Range<usize> = ID.start..ID.start + 1;
    pub(super) const FA: Range<usize> = INDEX.end..INDEX.end + 1;
}

mod experimental {
    use super::*;

    pub(super) const SN123: Range<usize> = EXPERIMENTAL.start..EXPERIMENTAL.start + 1;
    pub(super) const SN2: Range<usize> = SN123.end..SN123.end + 1;
}

mod calculated {
    use super::*;

    pub(super) const SN123: Range<usize> = CALCULATED.start..CALCULATED.start + 5;
    pub(super) const SN2: Range<usize> = SN123.end..SN123.end + 5;
    pub(super) const F: Range<usize> = SN2.end..SN2.end + 1;

    pub(super) mod sn123 {
        use super::*;

        pub(in super::super) const A: Range<usize> = SN123.start..SN123.start + 1;
        pub(in super::super) const B: Range<usize> = A.end..A.end + 1;
        pub(in super::super) const C: Range<usize> = B.end..B.end + 1;
        pub(in super::super) const D: Range<usize> = C.end..C.end + 1;
        pub(in super::super) const E: Range<usize> = D.end..D.end + 1;
    }

    pub(super) mod sn2 {
        use super::*;

        pub(in super::super) const A: Range<usize> = SN2.start..SN2.start + 1;
        pub(in super::super) const B: Range<usize> = A.end..A.end + 1;
        pub(in super::super) const C: Range<usize> = B.end..B.end + 1;
        pub(in super::super) const D: Range<usize> = C.end..C.end + 1;
        pub(in super::super) const E: Range<usize> = D.end..D.end + 1;
    }
}
