use crate::{age::Age, ID};
use serde_derive::{Deserialize, Serialize};

/// Instance of vehicle.
#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    pub id: String,
    /// Age periods available.
    pub age: Vec<Age>,
    /// MOV
    pub mov: u16,
    /// build
    pub build: u16,
    // todo
}

impl ID for Vehicle {
    fn id(&self) -> String {
        self.id.clone()
    }
}
