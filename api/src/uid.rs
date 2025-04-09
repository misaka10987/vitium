use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct UId(pub u64);

#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn test() -> UId {
    UId(12345678901234567890)
}
