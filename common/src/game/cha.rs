use super::{item::Armor, level::Level, Item, Ox, TypeName};
use crate::ID;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Cha {
    pub reg: Option<ID>,
    pub name: String,
    pub descr: String,
    pub race: ID,
    pub prof: ID,
    pub attr: HashMap<ID, Level>,
    pub skill: HashMap<ID, Level>,
    pub mart: HashMap<ID, Level>,
    pub spell: HashMap<ID, Level>,
    pub invt: Vec<Ox<Item>>,
    pub equip: Vec<Ox<Armor>>,
    pub money: i32,
}

impl AsRef<Option<ID>> for Cha {
    fn as_ref(&self) -> &Option<ID> {
        &self.reg
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PC {
    pub player: String,
    pub story: String,
    pub mods: HashSet<String>,
    pub cha: Cha,
}

impl TypeName for PC {
    fn typename() -> impl std::fmt::Display {
        "PlayerCharacter"
    }
}

impl Deref for PC {
    type Target = Cha;

    fn deref(&self) -> &Self::Target {
        &self.cha
    }
}

impl AsRef<Cha> for PC {
    fn as_ref(&self) -> &Cha {
        self.deref()
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub scena: usize,
    pub coord: (f32, f32),
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{test::*, ID};

    use super::Cha;

    impl Example for Cha {
        fn examples() -> Vec<Self> {
            vec![Self {
                reg: None,
                name: "Example Character".to_string(),
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
}
