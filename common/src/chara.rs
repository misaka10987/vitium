use crate::{
    item::Inventory,
    util::{Bottle, Envelop},
};
use serde_derive::{Deserialize, Serialize};

/// Defines attribution of a character.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub id: String,
    pub value: Envelop<Bottle<u16>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: i128,
    pub player: i128,
    pub name: String,
    pub descr: String,
    pub attr: Vec<Attr>,
    pub invt: Inventory,
}
