use crate::{age::Age, util::Envelop};
use serde_derive::{Deserialize, Serialize};

/// Defines a skill instance.
#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub growth: Envelop<u16>,
    pub profession: Envelop<u16>,
    pub interest: Envelop<u16>,
}

impl Skill {
    /// Sum up the skill profession, interest and growth level.
    pub fn level(&self) -> Envelop<u16> {
        if let Envelop::Open(g) = self.growth {
            if let Envelop::Open(p) = self.profession {
                if let Envelop::Open(i) = self.interest {
                    return Envelop::Open(g + p + i);
                }
            }
        }
        Envelop::Closed
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
