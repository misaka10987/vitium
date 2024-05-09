use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::Id;

use super::{item::ItemStore, PC};

/// A 1m*1m block with vertical height of 3m.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    /// Terrain in this block.
    pub terra: Id,
}

/// A 16*16-blocked chunk, used for lazy loading of the map.
#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// The blocks. Note that x and y coord is represented as `.block[x][y]`.
    pub block: Box<[[Block; 16]; 16]>,
}

/// Instance of scenario.
pub struct Scena {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    /// Loaded chunks.
    pub chunk: BTreeMap<(i16, i16), Chunk>,
    /// Player characters.
    pub pc: HashMap<String, PC>,
    pub item: ItemStore,
}
