use serde::{Deserialize, Serialize};

/// Edible item.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Edible {
    /// Whether the food tasts good, in [-100,100].
    pub taste: i8,
    /// How much energy the food can provide, in Joules.
    pub energy: i32,
    /// Whether the food has been processed and purified.
    pub purified: bool,
}
