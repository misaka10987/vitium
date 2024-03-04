use crate::ObjClass;
use serde::{Deserialize, Serialize};

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Prof {
    pub credit: u16,
    pub skills: Vec<(Skill, u16)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SkillAction {
    pub cat: ObjClass,
    pub obj: u64,
    pub skill: String,
}

impl SkillAction {
    pub fn example() -> Self {
        Self {
            cat: ObjClass::Mob,
            obj: 1145141919810,
            skill: "example-skill".to_string(),
        }
    }
}
