use super::{super::MARGIN, ID_SOURCE, Settings, State};
use crate::{
    app::{
        HashedMetaDataFrame, MAX_PRECISION,
        computers::{
            CalculationComputed, CalculationKey, DisplayComputed, DisplayKey, DisplayKind,
        },
        data::EMPTY_DATA_FRAME,
    },
    utils::{Hashed, hash_data_frame},
};
use egui::{
    Context, Frame, Grid, Id, Label, Margin, Popup, PopupCloseBehavior, Response, ScrollArea,
    Sense, TextStyle, TextWrapMode, Ui, Widget,
};
use egui_ext::{InnerResponseExt as _, ResponseExt as _};
use egui_phosphor::regular::{HASH, MINUS, PLUS};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use itertools::Itertools as _;
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::DataFrameExt as _;
use std::ops::Range;
use tracing::instrument;

const INDEX: Range<usize> = 0..1;
const TAG: Range<usize> = INDEX.end..INDEX.end + 1;
const LEN: usize = TAG.end;

/// Table view
pub(super) struct TableView<'a> {
    source: &'a mut [HashedMetaDataFrame],
    target: Hashed<DataFrame>,
    settings: &'a Settings,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(
        frames: &'a mut [HashedMetaDataFrame],
        settings: &'a Settings,
        state: &'a mut State,
    ) -> Self {
        Self {
            source: frames,
            target: EMPTY_DATA_FRAME.clone(),
            settings,
            state,
        }
    }
}

impl TableView<'_> {
    #[instrument(skip(self, ui), err)]
    pub(super) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        self.target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    frames: self.source,
                    parameters: &self.settings.parameters,
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
        let value = self.target.width() - 2;
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
            self.source[0].data.value.add_row()?;
            self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
            self.state.add_table_row = false;
        }
        if let Some(index) = self.state.delete_table_row {
            self.source[0].data.value.delete_row(index)?;
            self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
            self.state.delete_table_row = None;
        }
        if let Some(index) = self.state.take_firts_table_rows {
            self.source[0].data.value.firts_rows_to(index);
            self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
            self.state.add_table_row = false;
        }
        if let Some(index) = self.state.take_last_table_rows {
            self.source[0].data.value.last_rows_from(index);
            self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
            self.state.add_table_row = false;
        }
        Ok(())
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
                ui.heading("Triacylglycerol");
            }
            (0, _) => {
                ui.heading("Value");
            }
            // Bottom
            (1, INDEX) => {}
            (1, TAG) => {}
            (1, range) => {
                ui.heading(self.target[range.start].name().to_string());
            }
            //     ui.label(LayoutJob::subscripted_text(
            //         ui,
            //         "SN_1",
            //         Some(TextStyle::Heading),
            //         None,
            //     ));
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
            (row, &TAG) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<DisplayComputed>().get(DisplayKey {
                        data_frame: &self.target,
                        kind: DisplayKind::Composition {
                            composition: self.settings.parameters.composition,
                        },
                        percent: self.settings.percent,
                    })
                });
                if let Some(label) = data_frame[LABEL].str()?.get(row) {
                    let response = Label::new(label).sense(Sense::click()).ui(ui);
                    Popup::menu(&response)
                        .id(ui.auto_id_with("Species"))
                        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                        .show(|ui| species(ui, data_frame["Species"].list().unwrap(), row))
                        .transpose()?;
                }
            }
            (row, range) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<DisplayComputed>().get(DisplayKey {
                        data_frame: &self.target,
                        kind: DisplayKind::Value { index: range.start },
                        percent: self.settings.percent,
                    })
                });
                let value = data_frame.into_struct(PlSmallStr::EMPTY);
                if let Some(mean) = value.field_by_name("Mean")?.f64()?.get(row) {
                    let response = ui
                        .label(format!("{mean:.0$}", self.settings.precision))
                        .on_hover_text(mean.to_string());
                    // if response.hovered() {
                    //     response
                    //         .standard_deviation(&value, row)?
                    //         .repetitions(&value, row)?;
                    // }
                } else {
                    ui.label("-");
                }
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
            TAG => {}
            range => {
                let data_frame = ui.memory_mut(|memory| {
                    memory.caches.cache::<DisplayComputed>().get(DisplayKey {
                        data_frame: &self.target,
                        kind: DisplayKind::Value { index: range.start },
                        percent: self.settings.percent,
                    })
                });
                let value = data_frame.into_struct(PlSmallStr::EMPTY);
                if let Some(mean) = value.field_by_name("Mean")?.f64()?.sum() {
                    let response = ui
                        .label(format!("{mean:.0$}", self.settings.precision))
                        .on_hover_text(mean.to_string());
                    // if response.hovered() {
                    //     let standard_deviation = value.field_by_name("StandardDeviation")?;
                    //     if standard_deviation.f64()?.is_null().all() {
                    //         if let Some(standard_deviation) = standard_deviation.f64()?.sum() {
                    //             response.on_hover_text(format!("± {standard_deviation}"));
                    //         }
                    //     }
                    // }
                } else {
                    ui.label("-");
                }
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
                let _ = self.cell_content_ui(ui, cell.row_nr as _, cell.col_nr..cell.col_nr + 1);
            });
    }

    fn row_top_offset(&self, ctx: &Context, _table_id: Id, row: u64) -> f32 {
        row as f32 * (ctx.style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}

fn species(ui: &mut Ui, list: &ListChunked, row: usize) -> PolarsResult<()> {
    if let Some(species) = list.get_as_series(row) {
        ui.heading("Species")
            .on_hover_text(species.len().to_string());
        ui.separator();
        ScrollArea::vertical()
            // .auto_shrink([false, true])
            .show(ui, |ui| {
                Grid::new(ui.next_auto_id())
                    .show(ui, |ui| -> PolarsResult<()> {
                        for (index, (label, values)) in species
                            .struct_()?
                            .field_by_name(LABEL)?
                            .str()?
                            .iter()
                            .zip(species.struct_()?.field_by_name("Values")?.list()?)
                            .enumerate()
                        {
                            ui.label(index.to_string());
                            if let Some(label) = label {
                                ui.label(label);
                            }
                            if let Some(values) = values {
                                // // egui formated
                                // let number_formatter = ui.style().number_formatter.clone();
                                // let formated =
                                //     values.f64()?.iter().format_with(", ", |value, f| {
                                //         match value {
                                //             Some(value) => f(&number_formatter
                                //                 .format(value, 2..=MAX_PRECISION)),
                                //             None => f(&"-"),
                                //         }
                                //     });
                                // polars formated
                                let formated =
                                    values.iter().format_with(", ", |value, f| match value {
                                        AnyValue::Null => f(&"-"),
                                        value => f(&value),
                                    });
                                ui.label(format!("[{formated}]"));
                            }
                            ui.end_row();
                        }
                        Ok(())
                    })
                    .inner
            })
            .inner?;
    }
    Ok(())
}

/// Extension methods for [`Response`]
trait ResponseExt: Sized {
    fn species(self, list: &ListChunked, row: usize) -> PolarsResult<Self>;

    fn standard_deviation(self, r#struct: &StructChunked, row: usize) -> PolarsResult<Self>;

    fn repetitions(self, r#struct: &StructChunked, row: usize) -> PolarsResult<Self>;
}

impl ResponseExt for Response {
    fn species(mut self, list: &ListChunked, row: usize) -> PolarsResult<Self> {
        if let Some(species) = list.get_as_series(row) {
            self = self.try_on_enabled_hover_ui(|ui| -> PolarsResult<()> {
                // ui.heading("Species")
                //     .on_hover_text(species.len().to_string());
                ui.heading(species.len().to_string())
                    .on_hover_text(species.len().to_string());
                ui.separator();
                ScrollArea::both()
                    // .auto_shrink([false, true])
                    // .max_height(ui.spacing().combo_height)
                    .show(ui, |ui| {
                        Grid::new(ui.next_auto_id())
                            .show(ui, |ui| -> PolarsResult<()> {
                                for (index, (label, values)) in species
                                    .struct_()?
                                    .field_by_name(LABEL)?
                                    .str()?
                                    .iter()
                                    .zip(species.struct_()?.field_by_name("Values")?.list()?)
                                    .enumerate()
                                {
                                    ui.label(index.to_string());
                                    if let Some(label) = label {
                                        ui.label(label);
                                    }
                                    if let Some(values) = values {
                                        // // egui formated
                                        // let number_formatter = ui.style().number_formatter.clone();
                                        // let formated =
                                        //     values.f64()?.iter().format_with(", ", |value, f| {
                                        //         match value {
                                        //             Some(value) => f(&number_formatter
                                        //                 .format(value, 2..=MAX_PRECISION)),
                                        //             None => f(&"-"),
                                        //         }
                                        //     });
                                        // polars formated
                                        let formated = values.iter().format_with(
                                            ", ",
                                            |value, f| match value {
                                                AnyValue::Null => f(&"-"),
                                                value => f(&value),
                                            },
                                        );
                                        ui.label(format!("[{formated}]"));
                                    }
                                    ui.end_row();
                                }
                                Ok(())
                            })
                            .inner
                    })
                    .inner?;
                Ok(())
            })?;
        }
        Ok(self)
    }

    fn standard_deviation(mut self, r#struct: &StructChunked, row: usize) -> PolarsResult<Self> {
        if let Some(standard_deviation) =
            r#struct.field_by_name("StandardDeviation")?.f64()?.get(row)
        {
            self = self.on_hover_text(format!("± {standard_deviation}"));
        }
        Ok(self)
    }

    fn repetitions(mut self, r#struct: &StructChunked, row: usize) -> PolarsResult<Self> {
        if let Some(repetitions) = r#struct
            .field_by_name("Repetitions")?
            .array()?
            .get_as_series(row)
            && repetitions.len() > 1
        {
            let formated = repetitions.f64()?.iter().format_with(", ", |value, f| {
                if let Some(value) = value {
                    f(&value)?;
                }
                Ok(())
            });
            self = self.on_hover_text(format!("[{formated}]"));
        }
        Ok(self)
    }
}
