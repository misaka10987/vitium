use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub cost: i32,
}
