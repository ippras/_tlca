use super::ID_SOURCE;
use crate::{
    app::panes::MARGIN,
    markdown::{
        BHATTACHARYYA_COEFFICIENT, CHEBYSHEV_DISTANCE, COSINE_COEFFICIENT, EUCLIDEAN_DISTANCE,
        HELLINGER_COEFFICIENT, JACCARD_COEFFICIENT, JENSEN_SHANNON_COEFFICIENT, MANHATTAN_DISTANCE,
        PEARSON_CORRELATION_COEFFICIENT, SPEARMAN_RANK_CORRELATION_COEFFICIENT,
    },
    utils::AnyValueExt as _,
};
use egui::{
    Color32, Context, Frame, Grid, Id, Label, Margin, RichText, ScrollArea, TextStyle,
    TextWrapMode, Ui, Widget,
};
use egui_ext::Markdown as _;
use egui_phosphor::regular::INFO;
use egui_table::{AutoSizeMode, CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate};
use polars::prelude::*;
use std::ops::Range;
use tracing::instrument;

const METRICS: [&str; 8] = [
    "HellingerDistance",
    "JensenShannonDistance",
    "BhattacharyyaDistance",
    //
    "EuclideanDistance",
    "ChebyshevDistance",
    "ManhattanDistance",
    //
    "PearsonCorrelation",
    "SpearmanRankCorrelation",
];

const NAME: Range<usize> = 0..1;

/// Metrics
pub(crate) struct Metrics<'a> {
    pub(crate) data_frame: &'a DataFrame,
}

impl<'a> Metrics<'a> {
    pub(super) fn new(data_frame: &'a DataFrame) -> Self {
        Self { data_frame }
    }
}

impl Metrics<'_> {
    #[instrument(skip_all, err)]
    pub(crate) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Metrics");
        let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
        let (columns, rows) = self.data_frame.shape();
        TableBuilder::new()
            .id_salt(id_salt)
            .num_rows(rows as _)
            .columns(vec![Column::default().resizable(true); columns])
            .num_sticky_cols(1)
            .headers([HeaderRow::new(height)])
            .show(ui, self);
        Ok(())
    }

    fn header_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: Range<usize>) {
        ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        match (row, column) {
            (0, NAME) => {
                ui.heading("Name");
            }
            (0, column) => {
                ui.heading(self.data_frame[column.start].name().to_string());
            }
            _ => {}
        };
    }

    #[instrument(skip(self, ui), err)]
    fn body_cell_content_ui(
        &mut self,
        ui: &mut Ui,
        row: usize,
        column: Range<usize>,
    ) -> PolarsResult<()> {
        if let Some(value) = &self.data_frame[column.start].f64()?.get(row) {
            ui.label(value.to_string());
            // ui.label(format!("{mean:.0$}", self.settings.precision));
        }
        // match (row, column) {
        //     (row, NAME) => {
        //     }
        //     _ => {}
        // }
        Ok(())
    }
}

impl TableDelegate for Metrics<'_> {
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
                _ =
                    self.body_cell_content_ui(ui, cell.row_nr as _, cell.col_nr..cell.col_nr + 1);
            });
    }

    fn row_top_offset(&self, ctx: &Context, _table_id: Id, row: u64) -> f32 {
        row as f32 * (ctx.style().spacing.interact_size.y + 2.0 * MARGIN.y)
    }
}

// #[instrument(skip(self, ui), err)]
// pub(super) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
//     self.target = ui.memory_mut(|memory| {
//         memory
//             .caches
//             .cache::<CalculationComputed>()
//             .get(CalculationKey {
//                 frames: self.source,
//                 parameters: &self.settings.parameters,
//             })
//     });
//     let id_salt = Id::new(ID_SOURCE).with("Table");
//     if self.state.reset_table_state {
//         let id = TableState::id(ui, Id::new(id_salt));
//         TableState::reset(ui.ctx(), id);
//         self.state.reset_table_state = false;
//     }
//     let height = ui.text_style_height(&TextStyle::Heading) + 2.0 * MARGIN.y;
//     let num_rows = self.target.height() as u64 + 1;
//     let value = self.target.width() - 2;
//     let num_columns = LEN + value;
//     Table::new()
//         .id_salt(id_salt)
//         .num_rows(num_rows)
//         .columns(vec![
//             Column::default().resizable(self.settings.resizable);
//             num_columns
//         ])
//         .num_sticky_cols(self.settings.sticky)
//         .headers([
//             HeaderRow {
//                 height,
//                 groups: vec![INDEX, TAG, LEN..num_columns],
//             },
//             HeaderRow::new(height),
//         ])
//         .show(ui, self);
//     if self.state.add_table_row {
//         self.source[0].data.value.add_row()?;
//         self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
//         self.state.add_table_row = false;
//     }
//     if let Some(index) = self.state.delete_table_row {
//         self.source[0].data.value.delete_row(index)?;
//         self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
//         self.state.delete_table_row = None;
//     }
//     if let Some(index) = self.state.take_firts_table_rows {
//         self.source[0].data.value.firts_rows_to(index);
//         self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
//         self.state.add_table_row = false;
//     }
//     if let Some(index) = self.state.take_last_table_rows {
//         self.source[0].data.value.last_rows_from(index);
//         self.source[0].data.hash = hash_data_frame(&mut self.source[0].data.value)?;
//         self.state.add_table_row = false;
//     }
//     Ok(())
// }

// impl Widget for Statistics<'_> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         self.show(ui);
//         ui
//     }
// }

/// Extension methods for [`Ui`]
trait UiExt {
    fn header(&mut self, h1: &str, h2: &str);
}

impl UiExt for Ui {
    fn header(&mut self, h1: &str, h2: &str) {
        self.add(Label::new(h1).truncate());
        self.add(Label::new(RichText::new(h2).small()).truncate());
    }
}
