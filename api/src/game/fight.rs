use serde::{Deserialize, Serialize};

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub enum DmgType {
    Bashing,
    Slashing,
    Stabbing,
    Bleeding,
    Heat,
    Poison,
    Explosion,
    Starving,
    Thirst,
    Virus,
    Asphyxia,
    Mental,
    Magic,
    System,
}
