use crate::age::Age;
use serde_derive::{Deserialize, Serialize};

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
