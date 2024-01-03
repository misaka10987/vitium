use crate::{age::Age, armor::Armor, util::Envelop, weapon::Weapon};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Price = HashMap<Age, u64>;

#[derive(Serialize, Deserialize, Clone)]
pub struct OtherItem {
    pub uid: u128,
    pub name: String,
    pub descr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Other(OtherItem),
}

pub type Inventory = Vec<Envelop<Item>>;
