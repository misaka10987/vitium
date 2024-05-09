use std::collections::{HashMap, HashSet};

use crate::Id;

pub struct Race {
    /// Average height.
    pub height: u16,
    /// Average weight.
    pub weight: u16,
    /// Average life, in years.
    pub life: u16,
    /// Martial arts automatically learnt.
    pub mart: HashMap<Id, i16>,
    /// Spells automatically learnt.
    pub spell: HashSet<Id>,
}
