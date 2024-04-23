use crate::{DEBUG_DESCR, ID, UID};
use serde::{Deserialize, Serialize};

use super::{item::Item, Obj, Target};

/// Used for in-game chat.
#[derive(Clone, Serialize, Deserialize)]
pub struct Bubble {
    /// Content of the chat message.
    pub text: String,
    /// People(s) that you speak to, empty for towards all current people in the scenario.
    pub towards: Vec<usize>,
}

impl Bubble {
    pub fn example() -> Self {
        Self {
            text: DEBUG_DESCR.to_string(),
            towards: vec![114514, 1919810],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Action<'a> {
    Sync(SyncAction<'a>),
    Unsync(UnsyncAction),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SyncAction<'a> {
    Atk(Obj<'a>),
    Shoot(Obj<'a>),
    Cast(ID, Target<'a>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UnsyncAction {
    Move((f32, f32)),
    Travel(usize),
    Speak(Bubble),
    Consume(UID<Item>),
    Relax(usize),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Act<'a> {
    pub cha: String,
    pub action: Action<'a>,
}
