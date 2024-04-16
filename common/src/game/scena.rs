use std::{collections::BTreeMap, ops::Index};

use serde::{Deserialize, Serialize};

use crate::ID;

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
#[derive(Serialize, Deserialize, Clone)]
pub struct ScenaInst {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    pub chunk: BTreeMap<(i16, i16), Chunk>,
}
