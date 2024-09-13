pub mod armor;
pub mod container;
pub mod edible;
pub mod melee;
pub mod ranged;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use fe3o4::def_regtab;

/// Basic information of an item is stored here.
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
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
    pub flag: HashSet<String>,
}

def_regtab!(Item, R_ITEM);
