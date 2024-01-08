use crate::{player::Token, UID};
use serde_derive::{Deserialize, Serialize};

/// To cast `skill` on `object`.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cast {
    pub skill: String,
    pub object: i128,
}

/// All in-game actions are defined here.
#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    Move(i128),
    Wield(i128),
    Cast(Cast),
    Hello,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Act {
    pub uid: i128,
    pub chara: i128,
    pub turn: i128,
    pub action: Action,
    pub token: Token,
}

impl UID for Act {
    fn uid(&self) -> i128 {
        self.uid
    }

    fn set_uid(&mut self, uid: i128) -> &mut Self {
        self.uid = uid;
        self
    }
}
