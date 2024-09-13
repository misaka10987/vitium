use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use fe3o4::{def_regtab, Id};

use super::{Attr, Item, Mart, Spell};

/// Profession.
#[derive(Serialize, Deserialize, Clone)]
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
    /// Initial items given by this profession.
    pub item: Vec<Id<Item>>,
}

def_regtab!(Prof, REG_PROF);
