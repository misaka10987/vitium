use std::collections::HashSet;

use crate::{Bottle, Item, DEBUG_DESCR, UID};
use serde::{Deserialize, Serialize};

/// Defines attribution of a Chara.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub id: String,
    pub value: Bottle<i16>,
}

impl Attr {
    pub fn new(id: &str, value: Bottle<i16>) -> Self {
        Self {
            id: id.to_string(),
            value,
        }
    }
    pub fn fix(&self) -> i8 {
        ((self.value.now - 10) / 2) as i8
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chara {
    pub uid: u64,
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
            uid: 0,
            player: "debug-player".to_string(),
            name: "debug-chara".to_string(),
            descr: DEBUG_DESCR.to_string(),
            attr: vec![],
            invt: vec![],
            mods: HashSet::new(),
        }
    }
}

impl UID for Chara {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}
