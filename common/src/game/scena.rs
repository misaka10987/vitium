use std::{
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut, Index},
};

use serde::{Deserialize, Serialize};

use crate::{delta::Delta, tab::Tab, ID};

use super::{Cha, Item, PC};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub terra: ID,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub block: Vec<Block>,
}

impl Index<(i8, i8)> for Chunk {
    type Output = Block;

    fn index(&self, index: (i8, i8)) -> &Self::Output {
        let (x, y) = index;
        &self.block[((x % 16) * 16 + y % 16) as usize]
    }
}

/// Instance of scene.
pub struct ScenaInst<'a> {
    pub reg: &'a i8,
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    pub chunk: BTreeMap<(i16, i16), Chunk>,
    pub pc: HashMap<String, PC<'a>>,
    pub npc: Tab<'a, Cha<'a>>,
    pub item: Tab<'a, Item<'a>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackScena<'a> {
    pub item: Vec<<Tab<'a, Item<'a>> as Delta>::Pack>,
}

impl<'a> Delta for Scena<'a> {
    type Pack = PackScena<'a>;

    fn calc(&mut self) -> impl Iterator<Item = Self::Pack> {
        vec![Self::Pack {
            item: self.item.calc().collect(),
        }]
        .into_iter()
    }

    fn diff(&self) -> impl Iterator<Item = Self::Pack> {
        vec![Self::Pack {
            item: self.item.diff().collect(),
        }]
        .into_iter()
    }

    fn apply(&mut self, delta: impl Iterator<Item = Self::Pack>) {
        for i in delta {
            self.item.apply(i.item.into_iter());
        }
    }
}

pub struct Scena<'a> {
    pub inst: ScenaInst<'a>,
}

impl<'a> Deref for Scena<'a> {
    type Target = ScenaInst<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inst
    }
}

impl<'a> DerefMut for Scena<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inst
    }
}
