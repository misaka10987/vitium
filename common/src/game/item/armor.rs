use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{t_recs::Regis, Dice, Id};

/// Instance of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct Armor {
    /// Damage
    pub def: Dice,
    /// Species able to wear this armor.
    pub species: Species,
    /// Layers of the armor.
    pub layer: Vec<ArmorLayer>,
}

impl Regis for Armor {
    type Data = (i16,);
}

/// Defines a layer of armor.
#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorLayer {
    /// Material of this layer.
    pub mat: Id,
    /// Covering body parts.
    pub cover: HashSet<Id>,
    /// Covered rate.
    pub rate: f32,
    /// Thickness of material, in milimetres.
    pub thickness: i16,
}

/// Defines species for deciding if an armor is able to wear.
#[derive(Clone, Serialize, Deserialize)]
pub enum Species {
    /// Human-liked species.
    Human,
    /// Non human-liked species.
    NonHuman,
    /// Let host decide if able to wear.
    Else(String),
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{test::*, Id};

    use super::{Armor, ArmorLayer, Species};

    impl Example for Armor {
        fn examples() -> Vec<Self> {
            Species::examples()
                .into_iter()
                .map(|s| Self {
                    def: "11d45+14".to_string(),
                    species: s,
                    layer: ArmorLayer::examples(),
                })
                .collect()
        }
    }

    impl Example for ArmorLayer {
        fn examples() -> Vec<Self> {
            let mut cover = HashSet::new();
            cover.extend(Id::examples());
            vec![Self {
                mat: Id::example(),
                cover,
                rate: 0.95,
                thickness: 3000,
            }]
        }
    }

    impl Example for Species {
        fn examples() -> Vec<Self> {
            vec![Self::Human, Self::NonHuman, Self::Else("cat".to_string())]
        }
    }
}
