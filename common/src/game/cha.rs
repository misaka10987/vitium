use super::{level::Level, Item, TypeName};
use crate::ID;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

#[cfg(test)]
use crate::test::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Cha<'a> {
    pub reg: Option<ID>,
    pub name: String,
    pub descr: String,
    pub race: ID,
    pub prof: ID,
    pub attr: HashMap<ID, Level>,
    pub skill: HashMap<ID, Level>,
    pub mart: HashMap<ID, Level>,
    pub spell: HashMap<ID, Level>,
    pub invt: Vec<Cow<'a, Item<'a>>>,
    pub equip: Vec<Cow<'a, Item<'a>>>,
    pub money: i32,
}

impl<'a> AsRef<Option<ID>> for Cha<'a> {
    fn as_ref(&self) -> &Option<ID> {
        &self.reg
    }
}

#[cfg(test)]
impl<'a> Example for Cha<'a> {
    fn examples() -> Vec<Self> {
        vec![Self {
            reg: None,
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
pub struct PC<'a> {
    pub player: String,
    pub story: String,
    pub mods: HashSet<String>,
    pub cha: Cha<'a>,
}

impl<'a> TypeName for PC<'a> {
    fn typename() -> impl std::fmt::Display {
        "PlayerCharacter"
    }
}

impl<'a> AsRef<Cha<'a>> for PC<'a> {
    fn as_ref(&self) -> &Cha<'a> {
        &self.cha
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub scena: u64,
    pub coord: (f32, f32),
}
