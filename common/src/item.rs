use crate::{armor::Armor, envelop::Envelop, weapon::Weapon};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OtherItem {
    pub uid: u128,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Other(OtherItem),
}

pub type Inventory = Vec<Envelop<Item>>;
