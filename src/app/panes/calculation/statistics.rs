use crate::{
    markdown::{
        BHATTACHARYYA_COEFFICIENT, CHEBYSHEV_DISTANCE, COSINE_COEFFICIENT, EUCLIDEAN_DISTANCE,
        HELLINGER_COEFFICIENT, JACCARD_COEFFICIENT, JENSEN_SHANNON_COEFFICIENT, MANHATTAN_DISTANCE,
        PEARSON_CORRELATION_COEFFICIENT, SPEARMAN_RANK_CORRELATION_COEFFICIENT,
    },
    utils::AnyValueExt as _,
};
use egui::{Color32, Grid, Label, RichText, ScrollArea, Ui, Widget};
use egui_ext::Markdown as _;
use egui_phosphor::regular::INFO;
use polars::prelude::*;
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
    // #[instrument(skip(self, ui), err)]
    // pub(crate) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
    //     ScrollArea::both()
    //         .show(ui, |ui| {
    //             Grid::new(ui.next_auto_id())
    //                 .show(ui, |ui| -> PolarsResult<()> {
    //                     let columns = self.data_frame.get_columns();
    //                     // Header
    //                     ui.label("Metric");
    //                     for column in columns {
    //                         Label::new(column.name().as_str()).truncate().ui(ui);
    //                     }
    //                     ui.end_row();
    //                     // Separator
    //                     for _ in 0..=columns.len() {
    //                         ui.separator();
    //                     }
    //                     ui.end_row();
    //                     // Body
    //                     for metric in METRICS {
    //                         ui.horizontal(|ui| {
    //                             ui.menu_button(INFO, |ui| ui.markdown(EUCLIDEAN_DISTANCE));
    //                             ui.label(metric).on_hover_text(format!("{metric}.hover"));
    //                         });
    //                         for column in columns {
    //                             let text = &self.data_frame[column.name().as_str()]
    //                                 .struct_()?
    //                                 .field_by_name(metric)?
    //                                 .get(0)?
    //                                 .display();
    //                             ui.label(text);
    //                         }
    //                         // for name in METRICS {
    //                         //     // let name = field.name.as_str();
    //                         //     // ui.label(name).on_hover_text(format!("{name}.hover"));
    //                         //     let text = r#struct.field_by_name(name)?.get(0)?.display();
    //                         //     ui.label(text);
    //                         // }
    //                         ui.end_row();
    //                     }
    //                     // let euclidean_distance = self.data_frame["EuclideanDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(EUCLIDEAN_DISTANCE));
    //                     //     ui.label("Euclidean distance")
    //                     //         .on_hover_text("Евклидово расстояние");
    //                     // });
    //                     // ui.label(euclidean_distance);
    //                     // ui.end_row();
    //                     // let chebyshev_distance = self.data_frame["ChebyshevDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(CHEBYSHEV_DISTANCE));
    //                     //     ui.label("Chebyshev distance")
    //                     //         .on_hover_text("Расстояние Чебышёва");
    //                     // });
    //                     // ui.label(chebyshev_distance);
    //                     // ui.end_row();
    //                     // let manhattan_distance = self.data_frame["ManhattanDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(MANHATTAN_DISTANCE));
    //                     //     ui.label("Manhattan distance")
    //                     //         .on_hover_text("Манхэттенское расстояние");
    //                     // });
    //                     // ui.label(manhattan_distance);
    //                     // ui.end_row();
    //                     // ui.separator();
    //                     // ui.separator();
    //                     // ui.end_row();
    //                     // let bhattacharyya_distance =
    //                     //     self.data_frame["BhattacharyyaDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(BHATTACHARYYA_COEFFICIENT));
    //                     //     ui.label("BhattacharyyaDistance")
    //                     //         .on_hover_text("BhattacharyyaDistance.hover");
    //                     // });
    //                     // ui.label(bhattacharyya_distance);
    //                     // ui.end_row();
    //                     // let hellinger_distance = self.data_frame["HellingerDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(HELLINGER_COEFFICIENT));
    //                     //     ui.label("HellingerDistance")
    //                     //         .on_hover_text("HellingerDistance.hover");
    //                     // });
    //                     // ui.label(hellinger_distance);
    //                     // ui.end_row();
    //                     // let jensen_shannon_distance =
    //                     //     self.data_frame["JensenShannonDistance"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(JENSEN_SHANNON_COEFFICIENT));
    //                     //     ui.label("JensenShannonDistance")
    //                     //         .on_hover_text("JensenShannonDistance.hover");
    //                     // });
    //                     // ui.label(jensen_shannon_distance);
    //                     // ui.end_row();
    //                     // ui.separator();
    //                     // ui.separator();
    //                     // ui.end_row();
    //                     // let pearson_correlation = self.data_frame["PearsonCorrelation"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| ui.markdown(PEARSON_CORRELATION_COEFFICIENT));
    //                     //     ui.label("PearsonCorrelation")
    //                     //         .on_hover_text("PearsonCorrelation.hover");
    //                     // });
    //                     // ui.label(pearson_correlation);
    //                     // ui.end_row();
    //                     // let spearman_rank_correlation =
    //                     //     self.data_frame["SpearmanRankCorrelation"].get(0)?.display();
    //                     // ui.horizontal(|ui| {
    //                     //     ui.menu_button(INFO, |ui| {
    //                     //         ui.markdown(SPEARMAN_RANK_CORRELATION_COEFFICIENT)
    //                     //     });
    //                     //     ui.label("SpearmanRankCorrelation")
    //                     //         .on_hover_text("SpearmanRankCorrelation.hover");
    //                     // });
    //                     // ui.label(spearman_rank_correlation);
    //                     // ui.end_row();
    //                     Ok(())
    //                 })
    //                 .inner
    //         })
    //         .inner?;
    //     // let cosine_distance = self.data_frame["CosineDistance"].get(0)?.display();
    //     // let bray_curtis_dissimilarity =
    //     //     self.data_frame["BrayCurtisDissimilarity"].get(0)?.display();
    //     // let jaccard_distance = self.data_frame["JaccardDistance"].get(0)?.display();
    //     // Grid::new(ui.next_auto_id()).show(ui, |ui| {
    //     //     ui.horizontal(|ui| {
    //     //         ui.menu_button(INFO, |ui| ui.markdown(COSINE_SIMILARITY));
    //     //         ui.label("Cosine distance")
    //     //             .on_hover_text("Косинусное расстояние");
    //     //     });
    //     //     ui.label(cosine_distance);
    //     //     ui.end_row();
    //     // });
    //     // Grid::new(ui.next_auto_id()).show(ui, |ui| {
    //     //     ui.horizontal(|ui| {
    //     //         ui.menu_button(INFO, |ui| ui.markdown(BRAY_CURTIS_DISSIMILARITY));
    //     //         ui.label("Bray-Curtis dissimilarity")
    //     //             .on_hover_text("Несходство Брея-Кёртиса");
    //     //     });
    //     //     ui.label(bray_curtis_dissimilarity);
    //     //     ui.end_row();
    //     //     ui.horizontal(|ui| {
    //     //         ui.menu_button(INFO, |ui| ui.markdown(JACCARD_DISTANCE));
    //     //         ui.label("Ruzicka distance")
    //     //             .on_hover_text("Ruzicka distance or weighted Jaccard distance")
    //     //             .on_hover_text("Расстояние Ружички");
    //     //     });
    //     //     ui.label(jaccard_distance);
    //     //     ui.end_row();
    //     // });
    //     // let jensen_shannon_distance = self.data_frame["JensenShannonDistance"].get(0)?.display();
    //     // Grid::new(ui.next_auto_id()).show(ui, |ui| {
    //     //     ui.horizontal(|ui| {
    //     //         ui.menu_button(INFO, |ui| ui.markdown(JENSEN_SHANNON_DISTANCE));
    //     //         ui.label("Jensen-Shannon distance")
    //     //             .on_hover_text("Jensen-Shannon distance")
    //     //             .on_hover_text("Расстояние Дженсена-Шеннона");
    //     //     });
    //     //     ui.label(jensen_shannon_distance);
    //     //     ui.end_row();
    //     // });
    //     Ok(())
    // }

    pub(crate) fn show(&mut self, ui: &mut Ui) -> PolarsResult<()> {
        ScrollArea::both()
            .show(ui, |ui| {
                Grid::new(ui.next_auto_id())
                    .show(ui, |ui| -> PolarsResult<()> {
                        // Header
                        ui.label("Metric");
                        for metric in METRICS {
                            ui.label(metric)
                                .on_hover_text(format!("{metric}.hover"))
                                .on_hover_ui(|ui| {
                                    ui.markdown(EUCLIDEAN_DISTANCE);
                                });
                        }
                        ui.end_row();
                        // Separator
                        for _ in 0..=METRICS.len() {
                            ui.separator();
                        }
                        ui.end_row();
                        // Body
                        for column in self.data_frame.get_columns() {
                            let name = column.name().as_str();
                            ui.label(name);
                            for metric in METRICS {
                                let text = &self.data_frame[name]
                                    .struct_()?
                                    .field_by_name(metric)?
                                    .get(0)?
                                    .display();
                                ui.label(text);
                            }
                            ui.end_row();
                        }
                        Ok(())
                    })
                    .inner
            })
            .inner?;
        // let cosine_distance = self.data_frame["CosineDistance"].get(0)?.display();
        // let bray_curtis_dissimilarity =
        //     self.data_frame["BrayCurtisDissimilarity"].get(0)?.display();
        // let jaccard_distance = self.data_frame["JaccardDistance"].get(0)?.display();
        // Grid::new(ui.next_auto_id()).show(ui, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.menu_button(INFO, |ui| ui.markdown(COSINE_SIMILARITY));
        //         ui.label("Cosine distance")
        //             .on_hover_text("Косинусное расстояние");
        //     });
        //     ui.label(cosine_distance);
        //     ui.end_row();
        // });
        // Grid::new(ui.next_auto_id()).show(ui, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.menu_button(INFO, |ui| ui.markdown(BRAY_CURTIS_DISSIMILARITY));
        //         ui.label("Bray-Curtis dissimilarity")
        //             .on_hover_text("Несходство Брея-Кёртиса");
        //     });
        //     ui.label(bray_curtis_dissimilarity);
        //     ui.end_row();
        //     ui.horizontal(|ui| {
        //         ui.menu_button(INFO, |ui| ui.markdown(JACCARD_DISTANCE));
        //         ui.label("Ruzicka distance")
        //             .on_hover_text("Ruzicka distance or weighted Jaccard distance")
        //             .on_hover_text("Расстояние Ружички");
        //     });
        //     ui.label(jaccard_distance);
        //     ui.end_row();
        // });
        // let jensen_shannon_distance = self.data_frame["JensenShannonDistance"].get(0)?.display();
        // Grid::new(ui.next_auto_id()).show(ui, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.menu_button(INFO, |ui| ui.markdown(JENSEN_SHANNON_DISTANCE));
        //         ui.label("Jensen-Shannon distance")
        //             .on_hover_text("Jensen-Shannon distance")
        //             .on_hover_text("Расстояние Дженсена-Шеннона");
        //     });
        //     ui.label(jensen_shannon_distance);
        //     ui.end_row();
        // });
        Ok(())
    }
}

// impl Widget for Statistics<'_> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         self.show(ui);
//         ui
//     }
// }

enum Kind {
    Separator,
    Name(&'static str),
}

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
