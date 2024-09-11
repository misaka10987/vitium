use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    game::{DmgType, Mart, Skill},
    Dice, Id,
};

/// Melee weapons.
#[derive(Clone, Serialize, Deserialize)]
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
