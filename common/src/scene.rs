use crate::{ID, UID};
use serde::{Deserialize, Serialize};

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    /// Automatically generated uid.
    pub uid: u64,
    /// String id.
    pub id: String,
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    /// Path for optional ascii-art image.
    pub ascii: Option<String>,
}

impl ID for Scene {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
}

impl UID for Scene {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}
