use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use fe3o4::{def_regtab, Id};

use super::{Mart, Spell};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Race {
    /// Average height.
    pub height: u16,
    /// Average weight.
    pub weight: u16,
    /// Average life, in years.
    pub life: u16,
    /// Martial arts automatically learnt.
    pub mart: HashMap<Id<Mart>, i16>,
    /// Spells automatically learnt.
    pub spell: HashSet<Id<Spell>>,
}

def_regtab!(Race, R_RACE);
