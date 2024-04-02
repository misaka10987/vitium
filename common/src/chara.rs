use crate::{Attr, Item, DEBUG_DESCR};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone)]
pub struct Chara {
    pub player: String,
    pub name: String,
    pub descr: String,
    pub attr: Vec<Attr>,
    pub invt: Vec<Item>,
    pub mods: HashSet<String>,
}

impl Chara {
    pub fn new() -> Self {
        Self {
            player: "debug-player".to_string(),
            name: "debug-chara".to_string(),
            descr: DEBUG_DESCR.to_string(),
            attr: vec![],
            invt: vec![],
            mods: HashSet::new(),
        }
    }
}
