use self::{
    Composition::*,
    Stereospecificity::{NonStereospecific, Stereospecific},
};
use serde::{Deserialize, Serialize};

pub const COMPOSITIONS: [Composition; 12] = [
    SPECIES_STEREO,
    SPECIES_POSITIONAL,
    SPECIES_MONO,
    TYPE_STEREO,
    TYPE_POSITIONAL,
    TYPE_MONO,
    MASS_STEREO,
    MASS_MONO,
    ECN_STEREO,
    ECN_MONO,
    UNSATURATION_STEREO,
    UNSATURATION_MONO,
];

// Mass composition, non-stereospecific, agregation
pub const MASS_MONO: Composition = Mass(NonStereospecific(Agregation));
// Mass composition, stereospecific
pub const MASS_STEREO: Composition = Mass(Stereospecific);

// Equivalent carbon number composition, non-stereospecific, agregation
pub const ECN_MONO: Composition = EquivalentCarbonNumber(NonStereospecific(Agregation));
// Equivalent carbon number composition, stereospecific
pub const ECN_STEREO: Composition = EquivalentCarbonNumber(Stereospecific);

// Species composition, non-stereospecific, permutation
pub const SPECIES_MONO: Composition = Species(NonStereospecific(Permutation { positional: false }));
// Species composition, non-stereospecific, permutation, positional
pub const SPECIES_POSITIONAL: Composition =
    Species(NonStereospecific(Permutation { positional: true }));
// Species composition, stereospecific
pub const SPECIES_STEREO: Composition = Species(Stereospecific);

// Type composition, non-stereospecific, permutation
pub const TYPE_MONO: Composition = Type(NonStereospecific(Permutation { positional: false }));
// Type composition, non-stereospecific, permutation, positional
pub const TYPE_POSITIONAL: Composition = Type(NonStereospecific(Permutation { positional: true }));
// Type composition, stereospecific
pub const TYPE_STEREO: Composition = Type(Stereospecific);

// Unsaturation composition, non-stereospecific, agregation
pub const UNSATURATION_MONO: Composition = Unsaturation(NonStereospecific(Agregation));
// Unsaturation composition, stereospecific
pub const UNSATURATION_STEREO: Composition = Unsaturation(Stereospecific);

/// Composition
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Composition {
    EquivalentCarbonNumber(Stereospecificity<Agregation>),
    Mass(Stereospecificity<Agregation>),
    Species(Stereospecificity<Permutation>),
    Type(Stereospecificity<Permutation>),
    Unsaturation(Stereospecificity<Agregation>),
}

impl Composition {
    pub fn new() -> Self {
        SPECIES_STEREO
    }
}

impl Composition {
    pub fn forward(&self) -> Self {
        match *self {
            SPECIES_STEREO => SPECIES_POSITIONAL,
            SPECIES_POSITIONAL => SPECIES_MONO,
            SPECIES_MONO => TYPE_STEREO,
            TYPE_STEREO => TYPE_POSITIONAL,
            TYPE_POSITIONAL => TYPE_MONO,
            TYPE_MONO => MASS_STEREO,
            MASS_STEREO => MASS_MONO,
            MASS_MONO => ECN_STEREO,
            ECN_STEREO => ECN_MONO,
            ECN_MONO => UNSATURATION_STEREO,
            UNSATURATION_STEREO => UNSATURATION_MONO,
            UNSATURATION_MONO => UNSATURATION_MONO,
        }
    }

    pub fn backward(&self) -> Self {
        match *self {
            SPECIES_STEREO => SPECIES_STEREO,
            SPECIES_POSITIONAL => SPECIES_STEREO,
            SPECIES_MONO => SPECIES_POSITIONAL,
            TYPE_STEREO => SPECIES_MONO,
            TYPE_POSITIONAL => TYPE_STEREO,
            TYPE_MONO => TYPE_POSITIONAL,
            MASS_STEREO => TYPE_MONO,
            MASS_MONO => MASS_STEREO,
            ECN_STEREO => MASS_MONO,
            ECN_MONO => ECN_STEREO,
            UNSATURATION_STEREO => ECN_MONO,
            UNSATURATION_MONO => UNSATURATION_STEREO,
        }
    }
}

impl Composition {
    pub fn text(&self) -> &'static str {
        match *self {
            MASS_MONO => "Composition_Mass_Monospecific",
            MASS_STEREO => "Composition_Mass_Stereospecific",
            ECN_MONO => "Composition_EquivalentCarbonNumber_Monospecific",
            ECN_STEREO => "Composition_EquivalentCarbonNumber_Stereospecific",
            SPECIES_MONO => "Composition_Species_Monospecific",
            SPECIES_POSITIONAL => "Composition_Species_Positionalspecific",
            SPECIES_STEREO => "Composition_Species_Stereospecific",
            TYPE_MONO => "Composition_Type_Monospecific",
            TYPE_POSITIONAL => "Composition_Type_Positionalspecific",
            TYPE_STEREO => "Composition_Type_Stereospecific",
            UNSATURATION_MONO => "Composition_Unsaturation_Monospecific",
            UNSATURATION_STEREO => "Composition_Unsaturation_Stereospecific",
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match *self {
            MASS_MONO => "Composition_Mass_Monospecific.hover",
            MASS_STEREO => "Composition_Mass_Stereospecific.hover",
            ECN_MONO => "Composition_EquivalentCarbonNumber_Monospecific.hover",
            ECN_STEREO => "Composition_EquivalentCarbonNumber_Stereospecific.hover",
            SPECIES_MONO => "Composition_Species_Monospecific.hover",
            SPECIES_POSITIONAL => "Composition_Species_Positionalspecific.hover",
            SPECIES_STEREO => "Composition_Species_Stereospecific.hover",
            TYPE_MONO => "Composition_Type_Monospecific.hover",
            TYPE_POSITIONAL => "Composition_Type_Positionalspecific.hover",
            TYPE_STEREO => "Composition_Type_Stereospecific.hover",
            UNSATURATION_MONO => "Composition_Unsaturation_Monospecific.hover",
            UNSATURATION_STEREO => "Composition_Unsaturation_Stereospecific.hover",
        }
    }
}

// impl Display for Composition {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match *self {
//             MASS_MONO => f.write_str("Mass-Monospecific"),
//             MASS_STEREO => f.write_str("Mass-Stereospecific"),
//             ECN_MONO => f.write_str("EquivalentCarbonNumber-Monospecific"),
//             ECN_STEREO => f.write_str("EquivalentCarbonNumber-Stereospecific"),
//             SPECIES_MONO => f.write_str("Species-Monospecific"),
//             SPECIES_POSITIONAL => f.write_str("Species-Positionalspecific"),
//             SPECIES_STEREO => f.write_str("Species-Stereospecific"),
//             TYPE_MONO => f.write_str("Type-Monospecific"),
//             TYPE_POSITIONAL => f.write_str("Type-Positionalspecific"),
//             TYPE_STEREO => f.write_str("Type-Stereospecific"),
//             UNSATURATION_MONO => f.write_str("Unsaturation-Monospecific"),
//             UNSATURATION_STEREO => f.write_str("Unsaturation-Stereospecific"),
//         }
//     }
// }

impl Default for Composition {
    fn default() -> Self {
        Self::new()
    }
}

/// Stereospecificity
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Stereospecificity<T> {
    Stereospecific,
    NonStereospecific(T),
}

/// Agregation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Agregation;

/// Permutation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Permutation {
    pub positional: bool,
}
