use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub display_name: String,
    pub profile: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoPlayerError(pub String);

impl Display for NoPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player[name={}] does not exist", self.0)
    }
}

impl Error for NoPlayerError {}
