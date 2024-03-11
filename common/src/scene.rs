use serde::{Deserialize, Serialize};

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    /// Path for optional ascii-art image.
    pub ascii: Option<String>,
}
