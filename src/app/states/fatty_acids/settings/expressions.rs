use const_format::formatcp;
use egui::Ui;
use egui_l20n::prelude::*;
use fatty_acid_expressions::r#const::{BIODIESEL, METABOLIC, NUTRITIONAL, SUM, ratio, sum};
use serde::{Deserialize, Serialize};
use widgets::settings::Array;

/// Expressions
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Expressions {
    pub(crate) sum: Array,
    pub(crate) ratio: Ratio,
}

impl Expressions {
    pub(crate) fn new() -> Self {
        Self {
            sum: Array::from(sum::SUMS),
            ratio: Ratio::new(),
        }
    }

    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(SUM))
                .on_hover_localized(formatcp!("{SUM}.hover"));
            self.sum.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(BIODIESEL))
                .on_hover_localized(formatcp!("{BIODIESEL}.hover"));
            self.ratio.biodiesel.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(METABOLIC))
                .on_hover_localized(formatcp!("{METABOLIC}.hover"));
            self.ratio.metabolic.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(NUTRITIONAL))
                .on_hover_localized(formatcp!("{NUTRITIONAL}.hover"));
            self.ratio.nutritional.show(ui);
        });
    }
}

/// Ratio
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Ratio {
    pub(crate) biodiesel: Array,
    pub(crate) metabolic: Array,
    pub(crate) nutritional: Array,
}

impl Ratio {
    fn new() -> Self {
        Self {
            biodiesel: Array::from(ratio::biodiesel::RATIOS),
            metabolic: Array::from(ratio::metabolic::RATIOS),
            nutritional: Array::from(ratio::nutritional::RATIOS),
        }
    }
}
