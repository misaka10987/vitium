use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

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

def_regtab!(Edible, R_ITEM_EDIBLE);
