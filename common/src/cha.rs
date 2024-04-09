use crate::{item, level::Level, Item, ID};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::test::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Char {
    Player(PC),
    NonPlayer(Character),
}

impl AsRef<Character> for Char {
    fn as_ref(&self) -> &Character {
        match self {
            Char::Player(pc) => pc.as_ref(),
            Char::NonPlayer(npc) => &npc,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub descr: String,
    pub race: ID,
    pub prof: ID,
    pub attr: HashMap<ID, Level>,
    pub skill: HashMap<ID, Level>,
    pub mart: HashMap<ID, Level>,
    pub spell: HashMap<ID, Level>,
    pub invt: Vec<Item>,
    pub equip: Vec<item::Armor>,
    pub money: i32,
}

#[cfg(test)]
impl Example for Character {
    fn examples() -> Vec<Self> {
        vec![Self {
            name: "example-character".to_string(),
            descr: DEBUG_DESCR.to_string(),
            race: ID::example(),
            prof: ID::example(),
            attr: HashMap::new(),
            skill: HashMap::new(),
            mart: HashMap::new(),
            spell: HashMap::new(),
            invt: vec![],
            equip: vec![],
            money: 114514,
        }]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub player: String,
    pub story: String,
    pub mods: HashSet<String>,
    pub chara: Character,
}

impl AsRef<Character> for PC {
    fn as_ref(&self) -> &Character {
        &self.chara
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub scena: u64,
    pub coord: (f32, f32),
}
