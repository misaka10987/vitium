pub mod armor;
pub mod container;
pub mod edible;
pub mod melee;
pub mod ranged;
#[cfg(test)]
mod test;

use self::{armor::Armor, container::Container, edible::Edible, melee::Melee, ranged::Ranged};

use crate::{
    t_recs::{reg::RegTab, store::btree::BTreeStore, Compon, Entity, Regis},
    with_btree_store, with_reg, Id,
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

pub struct ItemStore {
    r_base: &'static RegTab<BaseItem>,
    r_armor: &'static RegTab<Armor>,
    r_container: &'static RegTab<Container>,
    r_edible: &'static RegTab<Edible>,
    r_melee: &'static RegTab<Melee>,
    r_ranged: &'static RegTab<Ranged>,
    base: BTreeStore<Item>,
    armor: BTreeStore<Item, Armor>,
    container: BTreeStore<Item, Container>,
    edible: BTreeStore<Item, Edible>,
    melee: BTreeStore<Item, Melee>,
    ranged: BTreeStore<Item, Ranged>,
}

with_reg!(ItemStore, r_base, BaseItem);
with_reg!(ItemStore, r_armor, Armor);
with_reg!(ItemStore, r_container, Container);
with_reg!(ItemStore, r_edible, Edible);
with_reg!(ItemStore, r_melee, Melee);
with_reg!(ItemStore, r_ranged, Ranged);
with_btree_store!(ItemStore, base, Item);
with_btree_store!(ItemStore, armor, Item, Armor);
with_btree_store!(ItemStore, container, Item, Container);
with_btree_store!(ItemStore, edible, Item, Edible);
with_btree_store!(ItemStore, melee, Item, Melee);
with_btree_store!(ItemStore, ranged, Item, Ranged);
