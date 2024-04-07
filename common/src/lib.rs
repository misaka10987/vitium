pub mod act;
pub mod atk;
pub mod cha;
pub mod cmd;
pub mod config;
pub mod dice;
pub mod feature;
pub mod fight;
pub mod game;
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
pub mod util;
pub mod vehicle;

use serde::{Deserialize, Serialize};

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

#[cfg(test)]
use crate::test::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ObjClass {
    Item,
    Chara,
    Scene,
    Vehicle,
    Mob,
    NPC,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Target {
    Entity(u64),
    Pos(i16, i16),
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ID {
    pub module: String,
    pub id: String,
}

impl ID {
    pub fn new(module: &str, id: &str) -> Self {
        Self {
            module: module.to_string(),
            id: id.to_string(),
        }
    }
}

#[cfg(test)]
impl Example for ID {
    fn examples() -> Vec<Self> {
        vec![ID::new("example-module", "example-id")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
