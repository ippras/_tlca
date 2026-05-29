use crate::{
    app::{
        panes::MARGIN,
        states::fatty_acids::{ID_SOURCE, settings::Settings},
    },
    r#const::NAME,
};
use const_format::formatcp;
use egui::{Frame, Id, Label, Margin, Response, TextStyle, TextWrapMode, Ui, Widget};
use egui_l20n::prelude::*;
use egui_phosphor::regular::{BROWSERS, HASH};
use egui_table::{CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate, TableState};
use fatty_acid_expressions::r#const::SUM;
use lipid::r#const::{
    INDEX, STEREOSPECIFIC_NUMBERS2, STEREOSPECIFIC_NUMBERS13, STEREOSPECIFIC_NUMBERS123,
};
use polars::prelude::*;
use polars_ext::prelude::*;
use std::ops::Range;
use tracing::instrument;
use widgets::polars::array::Float64Array;

#[cfg(feature = "markdown")]
use egui::Popup;
#[cfg(feature = "markdown")]
use egui_ext::Markdown as _;

const NUM_COLUMNS: usize = top::STEREOSPECIFIC_NUMBERS2.end;

pub(crate) const EXPRESSION: &str = "Expression";
pub(crate) const STEREOSPECIFIC_NUMBER: &str = "StereospecificNumber";

const TOP: &[Range<usize>] = &[
    top::INDEX,
    top::NAME,
    top::STEREOSPECIFIC_NUMBERS123,
    top::STEREOSPECIFIC_NUMBERS13,
    top::STEREOSPECIFIC_NUMBERS2,
];

/// Expressions widget
pub struct Expressions<'a> {
    data_frame: &'a DataFrame,
    settings: &'a mut Settings,
}

impl<'a> Expressions<'a> {
    pub fn new(data_frame: &'a DataFrame, settings: &'a mut Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let id_salt = Id::new(ID_SOURCE).with("EXPRESSIONS").with(SUM);
        if self.settings.reset {
            let id = TableState::id(ui, Id::new(id_salt));
            TableState::reset(ui.ctx(), id);
            self.settings.reset = false;
        }
        let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
        let num_rows = self.data_frame.height() as _;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default().resizable(self.settings.resizable);
                NUM_COLUMNS
            ])
            .num_sticky_cols(self.settings.sticky_columns)
            .headers([HeaderRow {
                height,
                groups: TOP.to_vec(),
            }])
            .show(ui, self)
    }

    fn header(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        if self.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, top::INDEX) => {
                ui.heading(HASH).on_hover_localized(INDEX);
            }
            (0, top::NAME) => {
                ui.heading(ui.localize(formatcp!("{EXPRESSION}")))
                    .on_hover_ui(|ui| {
                        ui.localize(formatcp!("{EXPRESSION}.hover"));
                    });
            }
            (0, column) => {
                ui.heading(self.data_frame[column.start - 1].name().as_str());
            }
            _ => {}
        }
    }

    #[instrument(skip(self, ui), err)]
    fn body(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) -> PolarsResult<()> {
        match (row, column) {
            (row, top::INDEX) => {
                ui.label(row.to_string());
            }
            (row, top::NAME) => {
                ui.visuals_mut().button_frame = false;

                #[allow(unused_variables)]
                let response = ui.button(BROWSERS);
                let name = self.data_frame[NAME].str()?.get(row);
                #[cfg(feature = "markdown")]
                if let Some(name) = name {
                    Popup::menu(&response).show(|ui| {
                        // let t = ui.localize(asset(ui, name));
                        // println!("localize: {t}");
                        ui.markdown(&ui.localize(&format!("{name}.markdown")));
                    });
                }
                //
                let text = name.map(|name| ui.localize(name)).display().to_string();
                let mut label = Label::new(text);
                if self.settings.truncate {
                    label = label.truncate();
                }
                label.ui(ui);
            }
            (row, column) => {
                Float64Array::builder()
                    .series(self.data_frame[column.start - 1].as_materialized_series())
                    .row(row)
                    .mean(self.settings.mean.mean)
                    .standard_deviation(self.settings.mean.standard_deviation)
                    .relative(self.settings.mean.kind.is_relative())
                    .build()
                    .show(ui)?;
            }
            // (row, top::STEREOSPECIFIC_NUMBERS123) => {
            //     Float64Array::builder()
            //         .series(self.data_frame[STEREOSPECIFIC_NUMBERS123].as_materialized_series())
            //         .row(row)
            //         .mean(self.settings.mean.mean)
            //         .standard_deviation(self.settings.mean.standard_deviation)
            //         .relative(self.settings.mean.kind.is_relative())
            //         .build()
            //         .show(ui)?;
            // }
            // (row, top::STEREOSPECIFIC_NUMBERS13) => {
            //     Float64Array::builder()
            //         .series(self.data_frame[STEREOSPECIFIC_NUMBERS13].as_materialized_series())
            //         .row(row)
            //         .mean(self.settings.mean.mean)
            //         .standard_deviation(self.settings.mean.standard_deviation)
            //         .relative(self.settings.mean.kind.is_relative())
            //         .build()
            //         .show(ui)?;
            // }
            // (row, top::STEREOSPECIFIC_NUMBERS2) => {
            //     Float64Array::builder()
            //         .series(self.data_frame[STEREOSPECIFIC_NUMBERS2].as_materialized_series())
            //         .row(row)
            //         .mean(self.settings.mean.mean)
            //         .standard_deviation(self.settings.mean.standard_deviation)
            //         .relative(self.settings.mean.kind.is_relative())
            //         .build()
            //         .show(ui)?;
            // }
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl TableDelegate for Expressions<'_> {
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
}

impl Widget for Expressions<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

mod top {
    use super::*;

    pub(super) const INDEX: Range<usize> = 0..1;
    pub(super) const NAME: Range<usize> = INDEX.end..INDEX.end + 1;
    pub(super) const STEREOSPECIFIC_NUMBERS123: Range<usize> = NAME.end..NAME.end + 1;
    pub(super) const STEREOSPECIFIC_NUMBERS13: Range<usize> =
        STEREOSPECIFIC_NUMBERS123.end..STEREOSPECIFIC_NUMBERS123.end + 1;
    pub(super) const STEREOSPECIFIC_NUMBERS2: Range<usize> =
        STEREOSPECIFIC_NUMBERS13.end..STEREOSPECIFIC_NUMBERS13.end + 1;
}
