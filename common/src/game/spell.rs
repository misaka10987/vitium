use serde::{Deserialize, Serialize};

use crate::{impl_reg, ID};

#[derive(Clone, Serialize, Deserialize)]
pub struct Spell {
    reg: Option<ID>,
    pub name: String,
    pub cost: i32,
}

impl_reg!(Spell);
