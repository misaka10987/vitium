use super::{level::Level, TypeName};
use crate::Id;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Cha {
    pub reg: Option<Id>,
    pub name: String,
    pub descr: String,
    pub pos: Pos,
    pub race: Id,
    pub prof: Id,
    pub attr: HashMap<Id, Level>,
    pub skill: HashMap<Id, Level>,
    pub mart: HashMap<Id, Level>,
    pub spell: HashMap<Id, Level>,
    // pub invt: Vec<Ox<Item>>,
    // pub equip: Vec<Ox<Armor>>,
    pub money: i32,
}

impl AsRef<Option<Id>> for Cha {
    fn as_ref(&self) -> &Option<Id> {
        &self.reg
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PC {
    pub player: String,
    pub story: String,
    pub mods: HashSet<String>,
    cha: Cha,
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

impl DerefMut for PC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cha
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

    use crate::{game::cha::Pos, test::*, Id};

    use super::Cha;

    impl Example for Pos {
        fn examples() -> Vec<Self> {
            vec![Self {
                scena: 114514,
                coord: (114.514, 1919.810),
            }]
        }
    }

    impl Example for Cha {
        fn examples() -> Vec<Self> {
            vec![Self {
                reg: None,
                name: "Example Character".to_string(),
                descr: DEBUG_DESCR.to_string(),
                pos: Pos::example(),
                race: Id::example(),
                prof: Id::example(),
                attr: HashMap::new(),
                skill: HashMap::new(),
                mart: HashMap::new(),
                spell: HashMap::new(),
                // invt: vec![],
                // equip: vec![],
                money: 114514,
            }]
        }
    }
}
