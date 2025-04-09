use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
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
