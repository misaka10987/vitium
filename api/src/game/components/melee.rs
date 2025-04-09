use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use fe3o4::Id;

use crate::{
    game::{DmgType, Mart, Skill},
    Dice,
};

/// Melee weapons.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Melee {
    /// Damage dice.
    pub atk: HashMap<DmgType, Dice>,
    /// In milimetres.
    pub rng: i32,
    /// Whether this weapon is one-handed.
    pub one_hand: bool,
    /// Skills that give bonus to fighting with this weapon.
    pub skill: HashSet<Id<Skill>>,
    /// Martial arts that can be performed with this weapon.
    pub mart: HashSet<Id<Mart>>,
}
