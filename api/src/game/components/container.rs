use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

use crate::UId;

/// Containers.
#[derive(Clone, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Container {
    /// Time to store an item.
    pub time_cost: i32,
    /// In milimetres.
    pub length: i32,
    /// In mililitres.
    pub volume: i32,
    /// In grams.
    pub weight: i32,
    /// If the container is waterproof.
    pub waterproof: bool,
    pub content: Vec<UId>,
}
