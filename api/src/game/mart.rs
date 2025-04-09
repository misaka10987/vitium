use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Mart;

def_regtab!(Mart, R_MART);
