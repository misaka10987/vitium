use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{delta::Delta, tab::Tab, ID};

use super::{Cha, Item, PC};

/// A 1m*1m block with vertical height of 3m.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    /// Terrain in this block.
    pub terra: ID,
}

/// A 16*16-blocked chunk, used for lazy loading of the map.
#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// The blocks. Note that x and y coord is represented as `.block[x][y]`.
    pub block: Box<[[Block; 16]; 16]>,
}

/// Instance of scenario.
pub struct Scena<'a> {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    /// Loaded chunks.
    pub chunk: BTreeMap<(i16, i16), Chunk>,
    /// Player characters.
    pub pc: HashMap<String, PC>,
    /// Non-player characters.
    pub npc: Tab<'a, Cha>,
    /// Items.
    pub item: Tab<'a, Item>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackScena<'a> {
    pub item: Vec<<Tab<'a, Item> as Delta>::Pack>,
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
