use crate::r#const::{EM_DASH, MEAN, SAMPLE, STANDARD_DEVIATION};
use egui::{Color32, Response, TextWrapMode, Ui, WidgetText};
use egui_l20n::prelude::*;
use polars::prelude::*;
use polars_utils::format_list;

/// Mean and standard deviation widget
pub struct MeanAndStandardDeviation<'a> {
    data_frame: &'a DataFrame,
    column: usize,
    row: usize,
    standard_deviation: bool,
    sample: bool,
    color: Option<Color32>,
}

impl<'a> MeanAndStandardDeviation<'a> {
    pub fn new(data_frame: &'a DataFrame, column: usize, row: usize) -> Self {
        Self {
            data_frame,
            column,
            row,
            standard_deviation: true,
            sample: false,
            color: None,
        }
    }

    pub fn with_standard_deviation(self, standard_deviation: bool) -> Self {
        Self {
            standard_deviation,
            ..self
        }
    }

    pub fn with_sample(self, sample: bool) -> Self {
        Self { sample, ..self }
    }
}

impl MeanAndStandardDeviation<'_> {
    pub fn show(&self, ui: &mut Ui) -> PolarsResult<Response> {
        let mean_series = self.data_frame[self.column]
            .struct_()?
            .field_by_name(MEAN)?;
        let mean = mean_series.f64()?.get(self.row);
        let standard_deviation_series = self.data_frame[self.column]
            .struct_()?
            .field_by_name(STANDARD_DEVIATION)?;
        let standard_deviation = standard_deviation_series.f64()?.get(self.row);
        let mut text = match mean {
            Some(mean)
                if self.standard_deviation
                    && let Some(standard_deviation) = standard_deviation =>
            {
                WidgetText::from(format!("{mean} ±{standard_deviation}"))
            }
            Some(mean) => WidgetText::from(mean.to_string()),
            None => WidgetText::from(EM_DASH),
        };
        if let Some(color) = self.color {
            text = text.color(color);
        }
        let mut response = ui.label(text);
        if response.hovered() {
            // Standard deviation
            if let Some(standard_deviation) = standard_deviation {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(STANDARD_DEVIATION));
                    ui.label(format!("±{standard_deviation}"));
                });
            }
            // Sample
            let sample_series = self.data_frame[self.column]
                .struct_()?
                .field_by_name(SAMPLE)?;
            if let Some(sample) = sample_series.array()?.get_as_series(self.row)
                && self.sample
            {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(SAMPLE));
                    ui.label(format_list!(sample.iter()));
                });
            }
        }
        Ok(response)
    }
}
