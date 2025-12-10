use crate::r#const::markdown::*;
use egui_phosphor::regular::{EXCLUDE, INTERSECT, UNITE};
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Sort {
    Key,
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
            Self::OverlapDistance => Self::OverlapDistance,
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
        }
    }
}

pub mod fatty_acids;
pub mod triacylglycerols;
