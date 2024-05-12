use std::collections::HashMap;

use super::{item::ItemStore, Map, PC};

/// Instance of scenario.
pub struct Scena {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    pub pos: (f64, f64),
    pub map: Map,
    /// Player characters.
    pub pc: HashMap<String, PC>,
    pub item: ItemStore,
}
