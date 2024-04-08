pub mod act;
pub mod atk;
pub mod cha;
pub mod cmd;
pub mod config;
pub mod dice;
pub mod feature;
pub mod fight;
pub mod game;
pub mod id;
pub mod item;
pub mod level;
pub mod mart;
pub mod mat;
pub mod module;
pub mod player;
mod prelude;
pub mod prof;
pub mod race;
pub mod record;
pub mod reg;
pub mod req;
pub mod res;
pub mod scena;
pub mod skill;
pub mod spell;
pub mod sync;
pub mod terra;
pub mod test;
pub mod uid;
pub mod util;
pub mod vehicle;

use serde::{Deserialize, Serialize};
use vehicle::Vehicle;

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let id: ID = obj("\"homo:sapiens\"").unwrap();
        assert_eq!(id, ID::new("homo", "sapiens"));
    }
}
