use crate::{age::Age, bottle::Bottle, dice::Dice};
use serde_derive::{Deserialize, Serialize};

/// Instance of weapon.
#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon {
    /// Age periods available.
    pub age: Vec<Age>,
    /// Damage expression using dice, eg `1d4+1`.
    pub dmg: Dice,
    /// In milimetres, `0` for melee weapons.
    pub rng: u32,
    /// Whether to apply penetration.
    pub pntr: bool,
    /// Number of attacks able to inflict in a turn.
    pub per_turn: u8,
    /// Charges remaining.
    pub charge: Bottle<u8>,
    /// Charges used per attack.
    pub load: u8,
    // todo
    // pub price:
}
