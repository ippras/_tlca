use self::composition::{Composition, SPECIES_STEREO};
use crate::markdown::*;
use egui::emath::Float as _;
use egui_phosphor::regular::{EXCLUDE, INTERSECT, UNITE};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameters {
    pub composition: Composition,
    pub filter: Filter,
    pub threshold: f64,
    pub sort: Sort,
    pub metric: Metric,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            composition: SPECIES_STEREO,
            filter: Filter::Union,
            threshold: 0.0,
            sort: Sort::Value,
            metric: Metric::HellingerDistance,
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for Parameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.composition.hash(state);
        self.filter.hash(state);
        self.threshold.ord().hash(state);
        self.sort.hash(state);
        self.metric.hash(state);
    }
}

/// Filter
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub enum Filter {
    #[default]
    Intersection, // And
    Union,      // Or
    Difference, // Xor
}

impl Filter {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Intersection => INTERSECT,
            Self::Union => UNITE,
            Self::Difference => EXCLUDE,
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::Intersection => "Filter_Intersection",
            Self::Union => "Filter_Union",
            Self::Difference => "Filter_Difference",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::Intersection => "Filter_Intersection.hover",
            Self::Union => "Filter_Union.hover",
            Self::Difference => "Filter_Difference.hover",
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub enum Sort {
    Key,
    #[default]
    Value,
}

impl Sort {
    pub fn text(&self) -> &'static str {
        match self {
            Self::Key => "Sort_Key",
            Self::Value => "Sort_Value",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::Key => "Sort_Key.hover",
            Self::Value => "Sort_Value.hover",
        }
    }
}

/// Metric
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Metric {
    // Distance between two discrete probability distributions
    HellingerDistance,
    JensenShannonDistance,
    BhattacharyyaDistance,
    // Distance between two points
    EuclideanDistance,
    ChebyshevDistance,
    ManhattanDistance,
    // Distance between two series
    CosineDistance,
    JaccardDistance,
    OverlapDistance,
    // Correlation between two series
    PearsonCorrelation,
    SpearmanRankCorrelation,
}

impl Metric {
    pub fn is_finite(&self) -> bool {
        matches!(
            self,
            Metric::HellingerDistance
                | Metric::JensenShannonDistance
                | Metric::CosineDistance
                | Metric::JaccardDistance
                | Metric::OverlapDistance
                | Metric::PearsonCorrelation
                | Metric::SpearmanRankCorrelation
        )
    }
}

impl Metric {
    pub fn forward(&self) -> Self {
        match self {
            Self::HellingerDistance => Self::JensenShannonDistance,
            Self::JensenShannonDistance => Self::BhattacharyyaDistance,
            Self::BhattacharyyaDistance => Self::EuclideanDistance,
            Self::EuclideanDistance => Self::ChebyshevDistance,
            Self::ChebyshevDistance => Self::ManhattanDistance,
            Self::ManhattanDistance => Self::CosineDistance,
            Self::CosineDistance => Self::JaccardDistance,
            Self::JaccardDistance => Self::OverlapDistance,
            Self::OverlapDistance => Self::PearsonCorrelation,
            Self::PearsonCorrelation => Self::SpearmanRankCorrelation,
            Self::SpearmanRankCorrelation => Self::SpearmanRankCorrelation,
        }
    }

    pub fn backward(&self) -> Self {
        match self {
            Self::HellingerDistance => Self::HellingerDistance,
            Self::JensenShannonDistance => Self::HellingerDistance,
            Self::BhattacharyyaDistance => Self::JensenShannonDistance,
            Self::EuclideanDistance => Self::BhattacharyyaDistance,
            Self::ChebyshevDistance => Self::EuclideanDistance,
            Self::ManhattanDistance => Self::ChebyshevDistance,
            Self::CosineDistance => Self::ManhattanDistance,
            Self::JaccardDistance => Self::CosineDistance,
            Self::OverlapDistance => Self::JaccardDistance,
            Self::PearsonCorrelation => Self::OverlapDistance,
            Self::SpearmanRankCorrelation => Self::PearsonCorrelation,
        }
    }
}

impl Metric {
    pub fn text(&self) -> &'static str {
        match self {
            Self::HellingerDistance => "Metric_HellingerDistance",
            Self::JensenShannonDistance => "Metric_JensenShannonDistance",
            Self::BhattacharyyaDistance => "Metric_BhattacharyyaDistance",
            Self::EuclideanDistance => "Metric_EuclideanDistance",
            Self::ChebyshevDistance => "Metric_ChebyshevDistance",
            Self::ManhattanDistance => "Metric_ManhattanDistance",
            Self::CosineDistance => "Metric_CosineDistance",
            Self::JaccardDistance => "Metric_JaccardDistance",
            Self::OverlapDistance => "Metric_OverlapDistance",
            Self::PearsonCorrelation => "Metric_PearsonCorrelation",
            Self::SpearmanRankCorrelation => "Metric_SpearmanRankCorrelation",
        }
    }

    pub fn hover_markdown(&self) -> &'static str {
        match self {
            Self::HellingerDistance => HELLINGER_COEFFICIENT,
            Self::JensenShannonDistance => JENSEN_SHANNON_COEFFICIENT,
            Self::BhattacharyyaDistance => BHATTACHARYYA_COEFFICIENT,
            Self::EuclideanDistance => EUCLIDEAN_DISTANCE,
            Self::ChebyshevDistance => CHEBYSHEV_DISTANCE,
            Self::ManhattanDistance => MANHATTAN_DISTANCE,
            Self::CosineDistance => COSINE_COEFFICIENT,
            Self::JaccardDistance => JACCARD_COEFFICIENT,
            Self::OverlapDistance => OVERLAP_COEFFICIENT,
            Self::PearsonCorrelation => PEARSON_CORRELATION_COEFFICIENT,
            Self::SpearmanRankCorrelation => SPEARMAN_RANK_CORRELATION_COEFFICIENT,
        }
    }
}

pub mod composition;
