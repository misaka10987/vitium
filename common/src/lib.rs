pub mod act;
pub mod attack;
pub mod chara;
pub mod cmd;
pub mod config;
pub mod dice;
pub mod feature;
pub mod fight;
pub mod game;
pub mod item;
pub mod json;
pub mod module;
pub mod player;
mod prelude;
pub mod race;
pub mod record;
pub mod registry;
pub mod req;
pub mod res;
pub mod scene;
pub mod skill;
pub mod sync;
pub mod util;
pub mod vehicle;

use serde::{Deserialize, Serialize};

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

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
    Chara(u64),
    Mob(u64),
    NPC(u64),
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait UID {
    fn uid(&self) -> u64;
    fn set_uid(&mut self, uid: u64) -> &mut Self;
    fn no_uid(&self) -> bool {
        self.uid() == 0
    }
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
