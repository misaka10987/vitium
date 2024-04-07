use std::collections::HashSet;

use crate::{Obj, ID};
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
    pub obj: Obj,
    pub skill: String,
}
