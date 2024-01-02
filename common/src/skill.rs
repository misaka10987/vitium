use crate::envelop::Envelop;
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
