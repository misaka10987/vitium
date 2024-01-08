use crate::{item::Inventory, util::Bottle, UID};
use serde_derive::{Deserialize, Serialize};

/// Defines attribution of a character.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub id: String,
    pub value: Option<Bottle<u16>>,
}

impl Attr {
    pub fn new(id: &str, value: Option<Bottle<u16>>) -> Self {
        Self {
            id: id.to_string(),
            value,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    uid: i128,
    pub player: String,
    pub name: String,
    pub descr: String,
    pub attr: Vec<Attr>,
    pub invt: Inventory,
}

impl Character {
    pub fn new(player: &str, name: &str, descr: &str, attr: Vec<Attr>, invt: Inventory) -> Self {
        Self {
            uid: 0,
            player: player.to_string(),
            name: name.to_string(),
            descr: descr.to_string(),
            attr,
            invt,
        }
    }
}

impl UID for Character {
    fn uid(&self) -> i128 {
        self.uid
    }

    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}
