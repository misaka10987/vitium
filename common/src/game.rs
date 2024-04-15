pub mod cha;
pub mod fight;
pub mod item;
pub mod level;
pub mod mart;
pub mod mat;
pub mod prelude;
pub mod prof;
pub mod race;
pub mod reg;
pub mod scena;
pub mod skill;
pub mod spell;
pub mod terra;
pub mod vehicle;

use serde::{Deserialize, Serialize};

use crate::UID;

pub use self::prelude::*;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Obj {
    Item(UID<Item>),
    Char(UID<Char>),
    Scena(UID<Scena>),
    Vehicle(UID<Vehicle>),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Target {
    Entity(Obj),
    Pos(i16, i16),
}

/// Refers to the current game status.
pub struct GameStat {
    /// Whether the game is ongoing now.
    pub on: bool,
    /// All player characters in this game.
    pub chara: HashSet<String>,
    /// Active player characters and if they had submitted their action.
    pub active: HashMap<String, bool>,
    /// Whether it has a finished turn now.
    pub done: bool,
    /// Whether the game has ended.
    pub term: bool,
    /// Turn number the game has reached.
    pub turn: i64,
    /// Host player of this game.
    pub host: HashSet<String>,
    /// Current mods loaded.
    pub modlist: HashSet<String>,
}
