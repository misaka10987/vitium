use serde::{Deserialize, Serialize};

use crate::t_recs::{reg::RegTab, Regis};

/// Edible item.
#[derive(Clone, Serialize, Deserialize)]
pub struct Edible {
    /// Whether the food tasts good, in [-100,100].
    pub taste: i8,
    /// How much energy the food can provide, in Joules.
    pub energy: i32,
    /// Whether the food has been processed and purified.
    pub purified: bool,
}

impl Regis for Edible {
    type Data = ();
}

impl Default for RegTab<Edible> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod test {

    use crate::test::Example;

    use super::Edible;

    impl Example for Edible {
        fn examples() -> Vec<Self> {
            vec![Self {
                taste: 50,
                energy: 1000,
                purified: true,
            }]
        }
    }
}
