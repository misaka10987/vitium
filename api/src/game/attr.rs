use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Attr;

def_regtab!(Attr, REG_ATTR);
