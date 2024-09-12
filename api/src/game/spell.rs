use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[derive(Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub cost: i32,
}

def_regtab!(Spell, R_SPELL);
