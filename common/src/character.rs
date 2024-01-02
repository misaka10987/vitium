use crate::attr::Attr;
use crate::item::Inventory;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: i128,
    pub name: String,
    pub attr: Vec<Attr>,
    pub invt: Inventory,
}
