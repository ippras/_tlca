use self::composition::{Composition, SPECIES_STEREO};
use egui::emath::Float as _;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct Parameters {
    pub(crate) composition: Composition,
    pub(crate) filter: Filter,
    pub(crate) threshold: f64,
    pub(crate) sort: Sort,
}

impl Parameters {
    pub(crate) fn new() -> Self {
        Self {
            composition: SPECIES_STEREO,
            filter: Filter::And,
            threshold: 0.0,
            sort: Sort::Value,
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
    }
}

/// Filter
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Filter {
    #[default]
    And, // And (Intersection)
    Or,  // Or (Union)
    Xor, // Xor
}

impl Filter {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::And => "And",
            Self::Or => "Or",
            Self::Xor => "Xor",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::And => "And.hover",
            Self::Or => "Or.hover",
            Self::Xor => "Xor.hover",
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) enum Sort {
    Key,
    #[default]
    Value,
}

impl Sort {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Key => "Key",
            Self::Value => "Value",
        }
    }

    pub(crate) fn hover_text(&self) -> &'static str {
        match self {
            Self::Key => "Key.hover",
            Self::Value => "Value.hover",
        }
    }
}

pub mod composition;
