use crate::Id;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Profession.
#[derive(Serialize, Deserialize, Clone)]
pub struct Prof {
    /// Coefficient of money for an initial character, timed by level.
    pub credit: u16,
    /// Attribution bonus provided by this profession.
    pub attr_bonus: HashMap<Id, i16>,
    /// Skills which this professions provides bonus.
    pub skill_bonus: HashMap<Id, i16>,
    /// Martial arts automatically learnt.
    pub mart: HashMap<Id, i16>,
    /// Spells automatically learnt.
    pub spell: HashSet<Id>,
    /// Initial items given by this profession.
    pub item: Vec<Id>,
}
