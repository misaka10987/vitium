pub mod act;
pub mod cha;
pub mod error;
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
pub mod world;

use serde::{Deserialize, Serialize};

use crate::UID;

pub use self::prelude::*;

use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Obj<'a> {
    Item(UID<Item>),
    Char(UID<Cha>),
    PC(UID<PC>),
    Scena(UID<ScenaInst<'a>>),
    Vehicle(UID<Vehicle>),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Target<'a> {
    Entity(Obj<'a>),
    Pos(i16, i16),
}

pub trait TypeName {
    fn typename() -> impl Display;
}

/// Refers to the current game status.
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
