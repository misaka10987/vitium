use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

/// Hitpoint.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Component)]
#[repr(transparent)]
#[serde(transparent)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct HP(pub i32);
