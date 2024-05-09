pub mod armor;
pub mod container;
pub mod edible;
pub mod melee;
pub mod ranged;
#[cfg(test)]
mod test;

use self::{armor::Armor, container::Container, edible::Edible, melee::Melee, ranged::Ranged};

use crate::{
    t_recs::{Compon, Entity, Regis},
    Id,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub base: BaseItem,
    pub armor: Option<Compon<Armor>>,
    pub container: Option<Compon<Container>>,
    pub edible: Option<Compon<Edible>>,
    pub melee: Option<Compon<Melee>>,
    pub ranged: Option<Compon<Ranged>>,
}

impl Entity for Item {
    type Base = BaseItem;
}

/// Basic information of an item is stored here.
#[derive(Clone, Serialize, Deserialize)]
pub struct BaseItem {
    pub name: String,
    pub descr: String,
    /// In milimetres.
    pub length: i32,
    /// In mililitres.
    pub volume: i32,
    /// In grams.
    pub weight: i32,
    /// If the item is opaque.
    pub opaque: bool,
    /// In the smallest currency unit, like 1 USD cent.
    pub price: i32,
    /// Extended information displayed.
    pub ext_info: Vec<String>,
    /// Flags.
    pub flag: HashSet<Id>,
}

impl Regis for BaseItem {
    type Data = ();
}
