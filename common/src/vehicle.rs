use serde_derive::{Deserialize, Serialize};

use crate::age::Age;
/// Instance of vehicle.
#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    /// Age periods available.
    pub age: Vec<Age>,
    /// MOV
    pub mov: u16,
    /// build
    pub build: u16,
    // todo
}