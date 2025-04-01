use super::{level::Level, Attr, Mart, Prof, Race, Skill, Spell};
use fe3o4::Id;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

/// Defines a character.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Char {
    pub name: String,
    pub descr: String,
    pub pos: Pos,
    pub race: Id<Race>,
    pub prof: Id<Prof>,
    pub attr: HashMap<Id<Attr>, Level>,
    pub skill: HashMap<Id<Skill>, Level>,
    pub mart: HashMap<Id<Mart>, Level>,
    pub spell: HashMap<Id<Spell>, Level>,
    // pub invt: Vec<Ox<Item>>,
    // pub equip: Vec<Ox<Armor>>,
    pub money: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PC {
    pub player: String,
    pub story: String,
    pub mods: HashSet<String>,
    cha: Char,
}

impl Deref for PC {
    type Target = Char;

    fn deref(&self) -> &Self::Target {
        &self.cha
    }
}

impl DerefMut for PC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cha
    }
}

impl AsRef<Char> for PC {
    fn as_ref(&self) -> &Char {
        self.deref()
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Pos {
    pub scena: usize,
    pub coord: (f32, f32),
}
