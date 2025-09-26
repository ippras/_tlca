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
            let mut color = ui.style().visuals.text_color();
            if self.settings.parameters.metric.is_finite() {
                if self.settings.chaddock {
                    color = sign.chaddock().color(color);
                } else {
                    color = sign.color(color);
                }
            }
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
    fn chaddock(&self) -> Sign<Chaddock> {
        match *self {
            Sign::Negative(value) => Sign::Negative(Chaddock::from(value)),
            Sign::Zero => Sign::Zero,
            Sign::Positive(value) => Sign::Positive(Chaddock::from(value)),
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

impl Sign<Chaddock> {
    fn color(&self, source: Color32) -> Color32 {
        match self {
            Sign::Negative(Chaddock::VeryStrong) => source.lerp_to_gamma(Color32::RED, 1.0),
            Sign::Negative(Chaddock::Strong) => source.lerp_to_gamma(Color32::RED, 0.8),
            Sign::Negative(Chaddock::Moderate) => source.lerp_to_gamma(Color32::RED, 0.6),
            Sign::Negative(Chaddock::Weak) => source.lerp_to_gamma(Color32::RED, 0.4),
            Sign::Negative(Chaddock::VeryWeak) => source.lerp_to_gamma(Color32::RED, 0.2),
            Sign::Zero => source,
            Sign::Positive(Chaddock::VeryWeak) => source.lerp_to_gamma(Color32::BLUE, 0.2),
            Sign::Positive(Chaddock::Weak) => source.lerp_to_gamma(Color32::BLUE, 0.4),
            Sign::Positive(Chaddock::Moderate) => source.lerp_to_gamma(Color32::BLUE, 0.6),
            Sign::Positive(Chaddock::Strong) => source.lerp_to_gamma(Color32::BLUE, 0.8),
            Sign::Positive(Chaddock::VeryStrong) => source.lerp_to_gamma(Color32::BLUE, 1.0),
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

/// Chaddock
#[derive(Clone, Copy, Debug)]
enum Chaddock {
    VeryStrong,
    Strong,
    Moderate,
    Weak,
    VeryWeak,
}

impl From<f64> for Chaddock {
    fn from(value: f64) -> Self {
        match value.abs() {
            0.0..0.3 => Self::VeryWeak,
            0.3..0.5 => Self::Weak,
            0.5..0.7 => Self::Moderate,
            0.7..0.9 => Self::Strong,
            0.9.. => Self::VeryStrong,
            _ => unreachable!(),
        }
    }
}

// /// Extension methods for [`Ui`]
// trait UiExt {
//     fn header(&mut self, h1: &str, h2: &str);
// }

// impl UiExt for Ui {
//     fn header(&mut self, h1: &str, h2: &str) {
//         self.add(Label::new(h1).truncate());
//         self.add(Label::new(RichText::new(h2).small()).truncate());
//     }
// }
