use serde::{Deserialize, Serialize};

use crate::{regis, typename};

#[derive(Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub cost: i32,
}
typename!(Spell, "Spell");
regis!(Spell);
