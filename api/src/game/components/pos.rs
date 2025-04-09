use std::num::Wrapping;

use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

/// Defines coordinate for characters.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Pos {
    /// The scene the character is in.
    #[cfg_attr(target_family = "wasm", tsify(type = "bigint"))]
    pub scene: u64,
    /// Coordinate with respect to scene origin.
    pub coord: (f32, f32),
}

/// Define a position on the tilemap.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct TilePos(
    #[cfg_attr(target_family = "wasm", tsify(type = "number"))] pub Wrapping<i16>,
    #[cfg_attr(target_family = "wasm", tsify(type = "number"))] pub Wrapping<i16>,
);
