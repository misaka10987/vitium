use serde::{Deserialize, Serialize};

use crate::ID;

pub struct Block {
    pub id: ID,
}

/// Instance of scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct Scenario {
    /// Displayed name.
    pub name: String,
    /// Description showed when a character enters.
    pub descr: String,
    /// Path for optional ascii-art image.
    pub ascii: Option<String>,
}
