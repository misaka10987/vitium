use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DmgType {
    Bashing,
    Slashing,
    Stabbing,
    Bleeding,
    Heat,
    Poison,
    Explosion,
    Starving,
    Thirst,
    Virus,
    Asphyxia,
    Mental,
    Magic,
    System,
}
