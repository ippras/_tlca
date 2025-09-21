use super::{ID_SOURCE, state::Settings};
use crate::app::panes::MARGIN;
use egui::{Color32, Id, Label, RichText, TextStyle, TextWrapMode, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use tracing::instrument;

/// Metrics
pub struct Metrics<'a> {
    pub data_frame: &'a DataFrame,
    pub settings: &'a Settings,
}

impl<'a> Metrics<'a> {
    pub(super) fn new(data_frame: &'a DataFrame, settings: &'a Settings) -> Self {
        Self {
            data_frame,
            settings,
        }
    }
}

impl Metrics<'_> {
    #[instrument(skip_all, err)]
    pub fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let id_salt = Id::new(ID_SOURCE).with("Metrics");
        let height = ui.text_style_height(&TextStyle::Heading);
        let rows = self.data_frame.height();
        let columns = self.data_frame.width();
        ui.style_mut().wrap_mode = if self.settings.truncate {
            Some(TextWrapMode::Truncate)
        } else {
            Some(TextWrapMode::Extend)
        };
        TableBuilder::new(ui)
            .id_salt(id_salt)
            .striped(true)
            .resizable(true)
            .columns(Column::auto(), columns + 1)
            .header(height + 2.0 * MARGIN.y, |mut row| {
                row.col(|_ui| {});
                for name in self.data_frame.get_column_names_str() {
                    row.col(|ui| {
                        ui.heading(name);
                    });
                }
            })
            .body(|mut body| {
                body.ui_mut().style_mut().wrap_mode = Some(TextWrapMode::Extend);
                body.rows(height, rows, |mut row| {
                    let index = row.index();
                    row.col(|ui| {
                        ui.label(self.data_frame[index].name().as_str());
                    });
                    for column in 0..columns {
                        row.col(|ui| {
                            let _ = self.body_cell_content_ui(ui, index, column);
                        });
                    }
                });
            });
        Ok(())
    }

    #[instrument(skip(self, ui), err)]
    fn body_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) -> PolarsResult<()> {
        if let Some(value) = self.data_frame[column].f64()?.get(row) {
            let text = format!("{value:.0$}", self.settings.precision);
            let sign = Sign::from(value);
            let color = ui.style().visuals.text_color();
            let color = if self.settings.rank {
                sign.color(color)
            } else {
                sign.rank().color(color)
            };
            Label::new(RichText::new(text).color(color))
                .ui(ui)
                .on_hover_text(value.to_string())
                .on_hover_text(format!("{sign:?}"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Sign<T> {
    Negative(T),
    Zero,
    Positive(T),
}

impl Sign<f64> {
    fn rank(&self) -> Sign<Rank> {
        match *self {
            Sign::Negative(value) => Sign::Negative(Rank::from(value)),
            Sign::Zero => Sign::Zero,
            Sign::Positive(value) => Sign::Positive(Rank::from(value)),
        }
    }
}

impl Sign<f64> {
    fn color(&self, source: Color32) -> Color32 {
        match *self {
            Sign::Negative(value) => source.lerp_to_gamma(Color32::RED, value as _),
            Sign::Zero => source,
            Sign::Positive(value) => source.lerp_to_gamma(Color32::BLUE, value as _),
        }
    }
}

impl Sign<Rank> {
    fn color(&self, source: Color32) -> Color32 {
        match self {
            Sign::Negative(Rank::VeryStrong) => source.lerp_to_gamma(Color32::RED, 1.0),
            Sign::Negative(Rank::Strong) => source.lerp_to_gamma(Color32::RED, 0.8),
            Sign::Negative(Rank::Moderate) => source.lerp_to_gamma(Color32::RED, 0.6),
            Sign::Negative(Rank::Weak) => source.lerp_to_gamma(Color32::RED, 0.4),
            Sign::Negative(Rank::VeryWeak) => source.lerp_to_gamma(Color32::RED, 0.2),
            Sign::Zero => source,
            Sign::Positive(Rank::VeryWeak) => source.lerp_to_gamma(Color32::BLUE, 0.2),
            Sign::Positive(Rank::Weak) => source.lerp_to_gamma(Color32::BLUE, 0.4),
            Sign::Positive(Rank::Moderate) => source.lerp_to_gamma(Color32::BLUE, 0.6),
            Sign::Positive(Rank::Strong) => source.lerp_to_gamma(Color32::BLUE, 0.8),
            Sign::Positive(Rank::VeryStrong) => source.lerp_to_gamma(Color32::BLUE, 1.0),
        }
    }
}

impl From<f64> for Sign<f64> {
    fn from(value: f64) -> Self {
        if value < 0.0 {
            Sign::Negative(value.abs())
        } else if value > 0.0 {
            Sign::Positive(value.abs())
        } else {
            Sign::Zero
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rank {
    VeryStrong,
    Strong,
    Moderate,
    Weak,
    VeryWeak,
}

impl From<f64> for Rank {
    fn from(value: f64) -> Self {
        match value.abs() {
            0.0..0.2 => Rank::VeryWeak,
            0.2..0.4 => Rank::Weak,
            0.4..0.6 => Rank::Moderate,
            0.6..0.8 => Rank::Strong,
            0.8..=1.0 => Rank::VeryStrong,
            _ => unreachable!(),
        }
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
