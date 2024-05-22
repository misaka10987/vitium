use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    game::DmgType,
    t_recs::{reg::RegTab, Regis},
    Dice, Id,
};

/// Ranged weapons.
#[derive(Clone, Serialize, Deserialize)]
pub struct Ranged {
    pub atk: HashMap<DmgType, Dice>,
    /// In metres.
    pub rng: f32,
    /// The minute-of-angle accuracy.
    pub moa: f32,
    /// Moving speed of the bullet.
    pub speed: f32,
    /// Items that can be used to charge this weapon.
    pub charge_item: HashSet<Id>,
    /// How many charges can be stored.
    pub charge_lim: i16,
    /// Charges used per shot.
    pub per_shot: u8,
    /// Shots able to perform in a turn.
    pub freq: f32,
}

impl Regis for Ranged {
    type Data = RangedData;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RangedData {
    pub charge: Vec<(Id, i16)>,
}

impl Default for RegTab<Ranged> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::{game::DmgType, test::Example, Id};

    use super::{Ranged, RangedData};

    impl Example for Ranged {
        fn examples() -> Vec<Self> {
            vec![Self {
                atk: HashMap::from([(DmgType::System, "11d45+14".to_string())]),
                rng: 114.514,
                moa: 1.14514,
                speed: 114.514,
                charge_item: HashSet::from([Id::example()]),
                charge_lim: 114,
                per_shot: 2,
                freq: 2.0,
            }]
        }
    }

    impl Example for RangedData {
        fn examples() -> Vec<Self> {
            vec![Self {
                charge: vec![(Id::example(), 114)],
            }]
        }
    }
}
