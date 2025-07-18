use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use fe3o4::Id;

use super::{Attr, Mart, Spell};

/// Profession.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Prof {
    /// Coefficient of money for an initial character, timed by level.
    pub credit: u16,
    /// Attribution bonus provided by this profession.
    pub attr_bonus: HashMap<Id<Attr>, i16>,
    /// Skills which this professions provides bonus.
    pub skill_bonus: HashMap<Id<Attr>, i16>,
    /// Martial arts automatically learnt.
    pub mart: HashMap<Id<Mart>, i16>,
    /// Spells automatically learnt.
    pub spell: HashSet<Id<Spell>>,
    // /// Initial items given by this profession.
    // pub item: Vec<Id<Item>>,
}
