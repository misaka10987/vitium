pub mod attr;
pub mod components;
pub mod fight;
pub mod level;
pub mod mart;
pub mod mat;
pub mod prelude;
pub mod prof;
pub mod race;
pub mod skill;
pub mod spell;
pub mod terra;

use serde::{Deserialize, Serialize};

use crate::UId;

pub use self::prelude::*;

use std::collections::HashSet;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Obj {
    Item(UId),
    Char(UId),
    PC(UId),
    Scena(usize),
    Vehicle(UId),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Target {
    Entity(Obj),
    Pos(i16, i16),
}

/// Refers to the current game status.
#[derive(Clone, Serialize, Deserialize)]
pub struct GameStat {
    /// Whether the game is ongoing now.
    pub on: bool,
    /// All player characters in this game.
    pub chara: HashSet<String>,
    /// Whether it has a finished turn now.
    pub done: bool,
    /// Whether the game has ended.
    pub term: bool,
    /// Turn number the game has reached.
    pub turn: i64,
    /// Host player of this game.
    pub host: String,
    /// Current mods loaded.
    pub modlist: HashSet<String>,
}
