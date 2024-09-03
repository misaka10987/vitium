use serde::{Deserialize, Serialize};

use super::Item;

/// Containers.
#[derive(Clone, Serialize, Deserialize)]
pub struct Container {
    /// Time to store an item.
    pub time_cost: i32,
    /// In milimetres.
    pub length: i32,
    /// In mililitres.
    pub volume: i32,
    /// In grams.
    pub weight: i32,
    /// If the container is waterproof.
    pub waterproof: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ContainerData {
    pub inside: Vec<Item>,
}
