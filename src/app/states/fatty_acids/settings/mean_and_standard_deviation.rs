use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l20n::prelude::*;
use serde::{Deserialize, Serialize};

const DELTA_DEGREES_OF_FREEDOM: &str = "DeltaDegreesOfFreedom";
const MEAN: &str = "Mean";
const STANDARD_DEVIATION: &str = "StandardDeviation";

/// Mean and standard deviation
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct MeanAndStandardDeviation {
    pub(crate) mean: bool,
    pub(crate) standard_deviation: bool,
    pub(crate) ddof: u8,
}

impl MeanAndStandardDeviation {
    pub(crate) fn new() -> Self {
        Self {
            mean: false,
            standard_deviation: false,
            ddof: 1,
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(MEAN))
                .on_hover_localized(formatcp!("{MEAN}.hover"));
            ui.checkbox(&mut self.mean, ());
            if !self.mean {
                self.standard_deviation = false;
                ui.disable();
            }
            ui.label(ui.localize(STANDARD_DEVIATION))
                .on_hover_localized(formatcp!("{STANDARD_DEVIATION}.hover"));
            ui.checkbox(&mut self.standard_deviation, ());
        });
        if self.standard_deviation {
            self.ddof(ui);
        }
    }
}

impl MeanAndStandardDeviation {
    /// DDOF
    ///
    /// https://numpy.org/devdocs/reference/generated/numpy.std.html
    fn ddof(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{DELTA_DEGREES_OF_FREEDOM}.abbreviation")))
                .on_hover_localized(DELTA_DEGREES_OF_FREEDOM)
                .on_hover_localized(formatcp!("{DELTA_DEGREES_OF_FREEDOM}.hover"));
            Slider::new(&mut self.ddof, 0..=1)
                .update_while_editing(false)
                .ui(ui);
        });
    }
}
