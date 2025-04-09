use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{game::DmgType, Dice};

/// Ranged weapons.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Ranged {
    pub atk: HashMap<DmgType, Dice>,
    /// In metres.
    pub rng: f32,
    /// The minute-of-angle accuracy.
    pub moa: f32,
    /// Moving speed of the bullet.
    pub speed: f32,
    /// Items that can be used to charge this weapon.
    pub charge_item: HashSet<String>,
    /// How many charges can be stored.
    pub charge_lim: i16,
    /// Charges used per shot.
    pub per_shot: u8,
    /// Shots able to perform in a turn.
    pub freq: f32,
}
