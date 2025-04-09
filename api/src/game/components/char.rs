use super::Pos;
use crate::game::{level::Level, Attr, Mart, Prof, Race, Skill, Spell};
use bevy_ecs::component::Component;
use fe3o4::Id;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Defines a character.
#[derive(Clone, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Char {
    /// Displayed name of the character.
    pub name: String,
    /// Additional description for the character.
    pub descr: String,
    /// Current position.
    pub pos: Pos,
    /// Race.
    pub race: Id<Race>,
    /// Profession.
    pub prof: Id<Prof>,
    /// Current attributes.
    pub attr: HashMap<Id<Attr>, Level>,
    /// Current skill levels.
    pub skill: HashMap<Id<Skill>, Level>,
    /// Current martial art levels.
    pub mart: HashMap<Id<Mart>, Level>,
    /// Current spell levels.
    pub spell: HashMap<Id<Spell>, Level>,
    // pub invt: Vec<bevy_ecs>,
    // pub equip: Vec<Ox<Armor>>,
    /// Current money.
    pub money: i32,
}

/// Denotes a player character.
#[derive(Clone, Serialize, Deserialize, Component)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct PlayerChar {
    /// User the character belongs to.
    pub user: String,
    /// Background story for the character.
    pub bg_story: String,
    /// Mods to use with.
    pub mods: HashSet<String>,
}
