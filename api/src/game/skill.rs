use super::{Attr, Obj};
use crate::{def_regtab, Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    /// Bonus provided by profession.
    pub prof_bonus: i16,
    /// Bonus provided by race.
    pub race_bonus: i16,
    /// Attributions that can give bonus to this skill.
    pub attr: HashSet<Id<Attr>>,
}

def_regtab!(Skill, R_SKILL);

#[derive(Clone, Serialize, Deserialize)]
pub struct SkillAction {
    pub obj: Obj,
    pub skill: String,
}
