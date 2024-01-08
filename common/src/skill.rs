use crate::age::Age;
use serde_derive::{Deserialize, Serialize};

/// Defines a skill instance.
#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub growth: Option<u16>,
    pub profession: Option<u16>,
    pub interest: Option<u16>,
}

impl Skill {
    /// Sum up the skill profession, interest and growth level.
    pub fn level(&self) -> Option<u16> {
        if let Some(g) = self.growth {
            if let Some(p) = self.profession {
                if let Some(i) = self.interest {
                    return Some(g + p + i);
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Prof {
    pub age: Age,
    pub id: String,
    pub name: String,
    pub credit: u16,
    pub skills: Vec<(Skill, u16)>,
}
