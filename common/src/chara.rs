use std::collections::HashSet;

use crate::{Item, DEBUG_DESCR};
use serde::{Deserialize, Serialize};

/// Defines attribution of a Chara.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub base: i16,
    pub curr: i16,
}

impl Attr {
    pub fn new(base: i16) -> Self {
        Self { base, curr: base }
    }
    pub fn fix(&self, zero: i16, coef: i16) -> i16 {
        (self.curr - zero) / coef
    }
}

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
