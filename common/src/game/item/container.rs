use crate::t_recs::Regis;
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

impl Regis for Container {
    type Data = ContainerData;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ContainerData {
    pub inside: Vec<Item>,
}

#[cfg(test)]
mod test {
    use crate::test::Example;

    use super::{Container, ContainerData};

    impl Example for Container {
        fn examples() -> Vec<Self> {
            vec![Self {
                time_cost: 1000,
                length: 114,
                volume: 514,
                weight: 1919,
                waterproof: false,
            }]
        }
    }

    impl Example for ContainerData {
        fn examples() -> Vec<Self> {
            vec![Self { inside: vec![] }]
        }
    }
}
