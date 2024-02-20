use crate::{player::Token, UID};
use serde::{Deserialize, Serialize};

/// To cast `skill` on `object`.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cast {
    pub skill: String,
    pub object: u64,
}

/// All in-game actions are defined here.
#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    Move(u64),
    Wield(u64),
    Cast(Cast),
    Hello,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Act {
    pub uid: u64,
    pub chara: u64,
    pub turn: u64,
    pub action: Action,
    pub token: Token,
}

impl UID for Act {
    /// UID getter.
    fn uid(&self) -> u64 {
        self.uid
    }
    /// UID setter.
    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}
