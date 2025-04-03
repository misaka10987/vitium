use serde::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub display_name: String,
    pub profile: Option<String>,
}
