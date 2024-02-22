use crate::{age::Age, ID, UID};
use serde::{Deserialize, Serialize};

/// Instance of vehicle.
#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    pub uid: u64,
    pub id: Option<ID>,
    /// Age periods available.
    pub age: Vec<Age>,
    /// MOV
    pub mov: u16,
    /// build
    pub build: u16,
    // todo
}

impl UID for Vehicle {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}
