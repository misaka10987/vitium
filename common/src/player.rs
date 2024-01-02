use crate::envelop::Envelop;
use serde_derive::{Deserialize, Serialize};

/// Defines a real-world player.
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub uid: i128,
    pub id: String,
    pub pswd: Envelop<String>,
    pub name: String,
    pub profile: String,
}

impl Player {
    pub fn new(
        uid: i128,
        id: String,
        pswd: Envelop<String>,
        name: String,
        profile: String,
    ) -> Self {
        Self {
            uid,
            id,
            pswd,
            name,
            profile,
        }
    }
    pub fn no_pswd(&self) -> Player {
        Player {
            uid: self.uid.clone(),
            id: self.id.clone(),
            pswd: Envelop::Closed,
            name: self.name.clone(),
            profile: self.profile.clone(),
        }
    }
}

/// Used for authentication.
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub uid: i128,
    pub pswd: String,
}

impl From<Player> for Token {
    fn from(value: Player) -> Self {
        Token {
            uid: value.uid,
            pswd: {
                if let Envelop::Open(p) = value.pswd {
                    p
                } else {
                    panic!("[FATAL] Trying to create token without password!")
                }
            },
        }
    }
}
