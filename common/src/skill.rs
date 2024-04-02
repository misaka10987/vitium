use std::collections::HashSet;

use crate::{ObjClass, ID};
use serde::{Deserialize, Serialize};

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    /// Bonus provided by profession.
    pub prof_bonus: i16,
    /// Bonus provided by race.
    pub race_bonus: i16,
    /// Attributions that can give bonus to this skill.
    pub attr: HashSet<ID>,
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
