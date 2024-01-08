use crate::{ID, UID};
use serde_derive::{Deserialize, Serialize};

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    uid: i128,
    pub id: String,
    pub name: String,
    pub description: String,
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
