use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

use super::DmgType;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Mat {
    /// In g/cm^3.
    pub density: f32,
    /// Per-milimetre resistance to damage.
    pub resist: HashMap<DmgType, f32>,
}

def_regtab!(Mat, R_MAT);
