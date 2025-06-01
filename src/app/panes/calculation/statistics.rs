use crate::{
    markdown::{
        BRAY_CURTIS_DISSIMILARITY, CHEBYSHEV_DISTANCE, COSINE_DISTANCE, EUCLIDEAN_DISTANCE,
        JENSEN_SHANNON_DISTANCE, MANHATTAN_DISTANCE, PEARSON_DISTANCE, RUZICKA_DISTANCE,
        SPEARMAN_DISTANCE,
    },
    utils::{AnyValueExt as _, UiExt as _},
};
use egui::{Grid, Label, RichText, Ui};
use egui_phosphor::regular::INFO;
use polars::prelude::*;
use tracing::instrument;

/// Statistics
pub(crate) struct Statistics<'a> {
    pub(crate) data_frame: &'a DataFrame,
}

impl<'a> Statistics<'a> {
    pub(super) fn new(data_frame: &'a DataFrame) -> Self {
        Self { data_frame }
    }
}

impl Statistics<'_> {
    #[instrument(skip(self, ui), err)]
    pub(crate) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        let cosine_distance = self.data_frame["CosineDistance"].get(0)?.display();
        let bray_curtis_dissimilarity =
            self.data_frame["BrayCurtisDissimilarity"].get(0)?.display();
        let ruzicka_distance = self.data_frame["RuzickaDistance"].get(0)?.display();

        ui.header(
            "Метрики, основанные на Lp-нормах",
            "Чувствительны к абсолютным значениям",
        );
        let euclidean_distance = self.data_frame["EuclideanDistance"].get(0)?.display();
        let chebyshev_distance = self.data_frame["ChebyshevDistance"].get(0)?.display();
        let manhattan_distance = self.data_frame["ManhattanDistance"].get(0)?.display();
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(EUCLIDEAN_DISTANCE));
                ui.label("Euclidean distance")
                    .on_hover_text("Евклидово расстояние");
            });
            ui.label(euclidean_distance);
            ui.end_row();

            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(CHEBYSHEV_DISTANCE));
                ui.label("Chebyshev distance")
                    .on_hover_text("Расстояние Чебышёва");
            });
            ui.label(chebyshev_distance);
            ui.end_row();

            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(MANHATTAN_DISTANCE));
                ui.label("Manhattan distance")
                    .on_hover_text("Манхэттенское расстояние");
            });
            ui.label(manhattan_distance);
            ui.end_row();
        });

        ui.header(
            "Метрики, основанные на схожести формы/профиля",
            "Менее чувствительны к абсолютным значениям, больше к относительным пропорциям",
        );
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(COSINE_DISTANCE));
                ui.label("Cosine distance")
                    .on_hover_text("Косинусное расстояние");
            });
            ui.label(cosine_distance);
            ui.end_row();
        });

        ui.header(
            "Метрики, учитывающие наличие/отсутствие и величины",
            "Часто используются в экологии и для сравнения распределений",
        );
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(BRAY_CURTIS_DISSIMILARITY));
                ui.label("Bray-Curtis dissimilarity")
                    .on_hover_text("Несходство Брея-Кёртиса");
            });
            ui.label(bray_curtis_dissimilarity);
            ui.end_row();
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(RUZICKA_DISTANCE));
                ui.label("Ruzicka distance")
                    .on_hover_text("Ruzicka distance or weighted Jaccard distance")
                    .on_hover_text("Расстояние Ружички");
            });
            ui.label(ruzicka_distance);
            ui.end_row();
        });

        ui.header(
            "Метрики для сравнения вероятностных распределений",
            "используют нормализованные векторы P, Q (вероятностные распределения)",
        );
        let jensen_shannon_distance = self.data_frame["JensenShannonDistance"].get(0)?.display();
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(JENSEN_SHANNON_DISTANCE));
                ui.label("Jensen-Shannon distance")
                    .on_hover_text("Jensen-Shannon distance")
                    .on_hover_text("Расстояние Дженсена-Шеннона");
            });
            ui.label(jensen_shannon_distance);
            ui.end_row();
        });

        ui.header(
            "Метрики, основанные на корреляции",
            "оценивают сходство формы/тренда",
        );
        let pearson_correlation = self.data_frame["PearsonCorrelation"].get(0)?.display();
        let spearman_correlation = self.data_frame["SpearmanCorrelation"].get(0)?.display();
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(PEARSON_DISTANCE));
                ui.label("Pearson correlation");
            });
            ui.label(pearson_correlation);
            ui.end_row();

            ui.horizontal(|ui| {
                ui.menu_button(INFO, |ui| ui.markdown(SPEARMAN_DISTANCE));
                ui.label("Spearman correlation");
            });
            ui.label(spearman_correlation);
            ui.end_row();
        });
        Ok(())
    }
}

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
