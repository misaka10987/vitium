use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{item, Item, ID};

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub player: String,
    pub name: String,
    pub descr: String,
    pub story: String,
    pub race: ID,
    pub prof: ID,
    pub attr: HashMap<ID, i16>,
    pub skill: HashMap<ID, i16>,
    pub martial: HashMap<ID, i16>,
    pub spell: HashSet<ID>,
    pub invt: Vec<Item>,
    pub equip: Vec<item::Armor>,
    pub money: i32,
    pub mods: HashSet<String>,
}
