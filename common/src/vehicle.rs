use crate::{age::Age, ID, UID};
use serde_derive::{Deserialize, Serialize};

/// Instance of vehicle.
#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    uid: i128,
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

impl UID for Vehicle {
    fn uid(&self) -> i128 {
        self.uid
    }

    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}
