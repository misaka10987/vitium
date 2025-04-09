use std::collections::HashSet;

use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

use fe3o4::Id;

use crate::{game::Mat, Dice};

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Armor {
    /// Defense.
    pub def: Dice,
    /// Layers of the armor.
    pub layer: Vec<ArmorLayer>,
}

/// Defines a layer of armor.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
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
