use serde_derive::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub profile: Option<String>,
}

impl Player {
    pub fn new(id: &str, name: &str, profile: Option<String>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            profile,
        }
    }
}

/// Used for authentication.
#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub id: String,
    pub pswd: String,
}

impl Token {
    pub fn new(id: &str, pswd: &str) -> Self {
        Self {
            id: id.to_string(),
            pswd: pswd.to_string(),
        }
    }
}
