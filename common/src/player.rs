use serde::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub display_name: String,
    pub profile: Option<String>,
}
