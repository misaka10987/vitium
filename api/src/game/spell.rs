use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Spell {
    pub name: String,
    pub cost: i32,
}

def_regtab!(Spell, R_SPELL);
