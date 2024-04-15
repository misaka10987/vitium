use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Spell {
    pub name: String,
    pub cost: i32,
}
