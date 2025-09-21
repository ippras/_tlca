use self::composition::{Composition, SPECIES_STEREO};
use crate::markdown::*;
use egui::emath::Float as _;
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
            filter: Filter::And,
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
    And, // And (Intersection)
    Or,  // Or (Union)
    Xor, // Xor
}

impl Filter {
    pub fn text(&self) -> &'static str {
        match self {
            Self::And => "And",
            Self::Or => "Or",
            Self::Xor => "Xor",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::And => "And.hover",
            Self::Or => "Or.hover",
            Self::Xor => "Xor.hover",
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
            Self::Key => "Key",
            Self::Value => "Value",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::Key => "Key.hover",
            Self::Value => "Value.hover",
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
    pub fn markdown(&self) -> &'static str {
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

impl Metric {
    pub fn text(&self) -> &'static str {
        match self {
            Self::HellingerDistance => "HellingerDistance",
            Self::JensenShannonDistance => "JensenShannonDistance",
            Self::BhattacharyyaDistance => "BhattacharyyaDistance",
            Self::EuclideanDistance => "EuclideanDistance",
            Self::ChebyshevDistance => "ChebyshevDistance",
            Self::ManhattanDistance => "ManhattanDistance",
            Self::CosineDistance => "CosineDistance",
            Self::JaccardDistance => "JaccardDistance",
            Self::OverlapDistance => "OverlapDistance",
            Self::PearsonCorrelation => "PearsonCorrelation",
            Self::SpearmanRankCorrelation => "SpearmanRankCorrelation",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::HellingerDistance => "HellingerDistance.hover",
            Self::JensenShannonDistance => "JensenShannonDistance.hover",
            Self::BhattacharyyaDistance => "BhattacharyyaDistance.hover",
            Self::EuclideanDistance => "EuclideanDistance.hover",
            Self::ChebyshevDistance => "ChebyshevDistance.hover",
            Self::ManhattanDistance => "ManhattanDistance.hover",
            Self::CosineDistance => "CosineDistance.hover",
            Self::JaccardDistance => "JaccardDistance.hover",
            Self::OverlapDistance => "OverlapDistance.hover",
            Self::PearsonCorrelation => "PearsonCorrelation.hover",
            Self::SpearmanRankCorrelation => "SpearmanRankCorrelation.hover",
        }
    }
}

pub mod composition;
