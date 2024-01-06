use crate::ID;
use serde_derive::{Deserialize, Serialize};

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl ID for Scene {
    fn id(&self) -> String {
        self.id.clone()
    }
}
