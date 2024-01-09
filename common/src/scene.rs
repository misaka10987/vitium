use crate::{ID, UID};
use serde_derive::{Deserialize, Serialize};

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    /// Automatically generated uid.
    pub uid: i128,
    /// String id.
    pub id: String,
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
}

impl ID for Scene {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl UID for Scene {
    fn uid(&self) -> i128 {
        self.uid
    }

    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}
