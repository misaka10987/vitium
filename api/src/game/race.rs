use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{def_regtab, Id};

use super::{Mart, Spell};

#[derive(Clone, Serialize, Deserialize)]
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
