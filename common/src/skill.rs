use std::collections::{HashMap, HashSet};

use crate::{Item, ObjClass, ID};
use serde::{Deserialize, Serialize};

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    /// Base level that is created with the character.
    pub base: i16,
    /// Growth level during the game.
    pub growth: i16,
    /// Bonus provided by profession.
    pub prof_bonus: i16,
    /// Bonus provided by race.
    pub race_bonus: i16,
    /// Attributions that can give bonus to this skill.
    pub attr: HashSet<ID>,
}

impl Skill {
    /// Sum up the skill base, profession and growth level.
    pub fn level(&self) -> i16 {
        self.base + self.growth + self.prof_bonus + self.race_bonus
    }
}

/// Profession.
#[derive(Serialize, Deserialize, Clone)]
pub struct Prof {
    /// Coefficient of money for an initial character, timed by level.
    pub credit: u16,
    /// Attribution bonus provided by this profession.
    pub attr_bonus: HashMap<ID, i16>,
    /// Skills which this professions provides bonus.
    pub skill_bonus: HashMap<ID, i16>,
    /// Initial items given by this profession.
    pub item: Vec<Item>,
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
