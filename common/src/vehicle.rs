use serde::{Deserialize, Serialize};

/// Instance of vehicle.
#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    /// MOV
    pub mov: u16,
    /// build
    pub build: u16,
    // todo
}
