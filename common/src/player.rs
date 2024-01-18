use crate::DEBUG_MSG;
use serde_derive::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub profile: Option<String>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: "debug-player".to_string(),
            name: "Debug Player".to_string(),
            profile: Some(DEBUG_MSG.to_string()),
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
    pub fn new() -> Self {
        Self {
            id: "debug-token".to_string(),
            pswd: "debug-pswd".to_string(),
        }
    }
}
