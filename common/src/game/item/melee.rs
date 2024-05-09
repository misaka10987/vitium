use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{game::DmgType, t_recs::Regis, Dice, Id};

/// Melee weapons.
#[derive(Clone, Serialize, Deserialize)]
pub struct Melee {
    /// Damage dice.
    pub atk: HashMap<DmgType, Dice>,
    /// In milimetres.
    pub rng: i32,
    /// Whether this weapon is one-handed.
    pub one_hand: bool,
    /// Skills that give bonus to fighting with this weapon.
    pub skill: HashSet<Id>,
    /// Martial arts that can be performed with this weapon.
    pub mart: HashSet<Id>,
}

impl Regis for Melee {
    type Data = (i16,);
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::{game::DmgType, test::Example, Id};

    use super::Melee;

    impl Example for Melee {
        fn examples() -> Vec<Self> {
            let mut atk = HashMap::new();
            atk.insert(DmgType::System, "11d45+14".to_string());
            let mut skill = HashSet::new();
            skill.extend(Id::examples());
            let mut mart = HashSet::new();
            mart.extend(Id::examples());
            vec![Self {
                atk,
                rng: 514,
                one_hand: true,
                skill,
                mart,
            }]
        }
    }
}
