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
pub mod uid;
pub mod util;
pub mod vehicle;

use scena::Scena;
use serde::{de::Visitor, Deserialize, Serialize};
use vehicle::Vehicle;

pub use crate::prelude::*;

pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

#[cfg(test)]
use crate::test::*;

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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ID {
    pub module: String,
    pub id: String,
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.module, self.id))
    }
}

struct IDVisitor;

impl<'de> Visitor<'de> for IDVisitor {
    type Value = ID;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string that can be splitted by a ':' into two valid identifiers"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let s: Vec<&str> = v.split(':').collect();
        if s.len() != 2 {
            return Err(E::missing_field("id"));
        }
        Ok(ID::new(s[0], s[1]))
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IDVisitor)
    }
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
        let x: ID = obj("\"homo:sapiens\"").unwrap();
        println!("{}", json(&x).unwrap());
    }
}
