use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use fe3o4::Id;

use crate::{game::Mat, Dice};

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct Armor {
    /// Damage
    pub def: Dice,
    /// Species able to wear this armor.
    pub species: Species,
    /// Layers of the armor.
    pub layer: Vec<ArmorLayer>,
}

/// Defines a layer of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorLayer {
    /// Material of this layer.
    pub mat: Id<Mat>,
    /// Covering body parts.
    pub cover: HashSet<String>,
    /// Covered rate.
    pub rate: f32,
    /// Thickness of material, in milimetres.
    pub thickness: i16,
}

/// Defines species for deciding if an armor is able to wear.
#[derive(Clone, Serialize, Deserialize)]
pub enum Species {
    /// Human-liked species.
    Human,
    /// Non human-liked species.
    NonHuman,
    /// Let host decide if able to wear.
    Else(String),
}
