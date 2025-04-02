use super::{Attr, Obj};
use fe3o4::{def_regtab, Id};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
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
