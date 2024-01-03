use serde_derive::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub uid: i128,
    pub id: String,
    pub name: String,
    pub profile: Option<String>,
}

impl Player {
    pub fn new(uid: i128, id: String, name: String, profile: Option<String>) -> Self {
        Self {
            uid,
            id,
            name,
            profile,
        }
    }
}

/// Used for authentication.
#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub uid: i128,
    pub pswd: String,
}
