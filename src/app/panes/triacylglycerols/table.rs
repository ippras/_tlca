use crate::{
    app::{
        computers::triacylglycerols::{
            Computed as TriacylglycerolsComputed, Key as TriacylglycerolsKey,
            table::{Computed as FormatComputed, Key as FormatKey},
        },
        panes::MARGIN,
        states::triacylglycerols::{ID_SOURCE, State},
        widgets::mean_and_standard_deviation::MeanAndStandardDeviation,
    },
    r#const::{SPECIES, THRESHOLD},
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::{
    Align, Context, Frame, Grid, Id, Label, Layout, Margin, Popup, PopupCloseBehavior, ScrollArea,
    Sense, TextStyle, TextWrapMode, Ui, Widget,
};
use egui_ext::{InnerResponseExt as _, ResponseExt};
use egui_l20n::prelude::*;
use egui_phosphor::regular::{HASH, LIST};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use lipid::prelude::*;
use polars::prelude::*;
use std::ops::Range;
use tracing::instrument;

/// Table view
pub(super) struct TableView<'a> {
    source: &'a mut [HashedMetaDataFrame],
    target: HashedDataFrame,
    state: &'a mut State,
}

impl<'a> TableView<'a> {
    pub(super) fn new(frames: &'a mut [HashedMetaDataFrame], state: &'a mut State) -> Self {
        Self {
            source: frames,
            target: HashedDataFrame::EMPTY,
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
                .cache::<TriacylglycerolsComputed>()
                .get(TriacylglycerolsKey::new(self.source, &self.state.settings))
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
        let num_columns = headers::LEN + value;
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
                    groups: vec![
                        headers::INDEX,
                        headers::TAG,
                        headers::LEN..num_columns - 1,
                        num_columns - 1..num_columns,
                    ],
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
            (0, headers::INDEX) => {
                ui.heading(HASH);
            }
            (0, headers::TAG) => {
                ui.heading(ui.localize(self.state.settings.composition.text()))
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(self.state.settings.composition.hover_text()));
                    });
            }
            (0, column) if column.end != self.target.width() => {
                ui.heading(ui.localize("Value"));
            }
            (0, _) => {
                ui.heading(ui.localize(SPECIES));
            }
            // Bottom
            (1, column)
                if !matches!(column, headers::INDEX | headers::TAG)
                    && column.end != self.target.width() =>
            {
                ui.heading(self.target[column.start].name().to_string());
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
        if let Some(threshold) = self.target[THRESHOLD].bool()?.get(row)
            && !threshold
        {
            ui.multiply_opacity(ui.visuals().disabled_alpha());
        }
        match (row, column) {
            (row, headers::INDEX) => {
                ui.label(row.to_string());
            }
            (row, headers::TAG) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<FormatComputed>()
                        .get(FormatKey::new(&self.target, &self.state.settings))
                });
                if let Some(label) = data_frame[LABEL].str()?.get(row) {
                    let response = Label::new(label).sense(Sense::click()).ui(ui);
                    Popup::menu(&response)
                        .id(ui.auto_id_with(SPECIES))
                        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                        .show(|ui| species(ui, &data_frame, row))
                        .transpose()?;
                }
            }
            (row, column) if column.end != self.target.width() => {
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<FormatComputed>()
                        .get(FormatKey::new(&self.target, &self.state.settings))
                });
                MeanAndStandardDeviation::new(&data_frame, column.start, row)
                    .with_standard_deviation(true)
                    .with_sample(true)
                    .show(ui)?;
            }
            (row, _last) => {
                let data_frame = ui.memory_mut(|memory| {
                    memory
                        .caches
                        .cache::<FormatComputed>()
                        .get(FormatKey::new(&self.target, &self.state.settings))
                });
                let response = ui.button(LIST).try_on_hover_ui(|ui| -> PolarsResult<()> {
                    if let Some(length) = data_frame[SPECIES].list()?.lst_lengths().get(row) {
                        ui.label(length.to_string());
                    }
                    Ok(())
                })?;
                Popup::menu(&response)
                    .id(ui.auto_id_with(SPECIES))
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .show(|ui| species(ui, &data_frame, row))
                    .transpose()?;
            }
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: Range<usize>) -> PolarsResult<()> {
        if !matches!(column, headers::INDEX | headers::TAG) && column.end != self.target.width() {
            // let data_frame = ui.memory_mut(|memory| {
            //     memory
            //         .caches
            //         .cache::<FormatComputed>()
            //         .get(FormatKey::new(&self.target, &self.state.settings))
            // });
            // let row = data_frame.height() - 1;
            // mean_and_standard_deviation(ui, &data_frame, row)?;
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

fn species(ui: &mut Ui, data_frame: &DataFrame, row: usize) -> PolarsResult<()> {
    if let Some(species) = data_frame[SPECIES].list()?.get_as_series(row) {
        ui.heading(SPECIES).on_hover_text(species.len().to_string());
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
                            .zip(species.struct_()?.field_by_name("Values")?.str()?)
                            .enumerate()
                        {
                            ui.label(index.to_string());
                            if let Some(label) = label {
                                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                    ui.set_max_width(ui.spacing().tooltip_width / 2.0);
                                    Label::new(label).truncate().ui(ui);
                                });
                            }
                            if let Some(values) = values {
                                ui.label(values);
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

mod headers {
    use super::*;

    pub(super) const INDEX: Range<usize> = 0..1;
    pub(super) const TAG: Range<usize> = INDEX.end..INDEX.end + 1;
    pub(super) const LEN: usize = TAG.end;
}
