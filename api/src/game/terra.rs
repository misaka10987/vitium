use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Terra {
    /// Symbol displayed on the map.
    pub sym: char,
    /// Time used to pass, in APs. `None` for terrains that block movement.
    pub mv_time: Option<i16>,
    /// [0,1], 0 for completely transparently and 1 for completely opaque.
    pub opaque: f32,
}

def_regtab!(Terra, R_TERRA);
