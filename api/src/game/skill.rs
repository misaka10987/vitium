use super::{Attr, Obj};
use fe3o4::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Defines a skill instance.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Skill {
    /// Bonus provided by profession.
    pub prof_bonus: i16,
    /// Bonus provided by race.
    pub race_bonus: i16,
    /// Attributions that can give bonus to this skill.
    pub attr: HashSet<Id<Attr>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SkillAction {
    pub obj: Obj,
    pub skill: String,
}
