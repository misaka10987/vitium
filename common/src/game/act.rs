use crate::DEBUG_DESCR;
use serde::{Deserialize, Serialize};

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
pub enum Action {
    Sync,
    RT,
}

pub enum RTAction {
    Move((f32, f32)),
    Travel(usize),
    Speak(Bubble),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Act {
    pub cha: String,
    pub action: Action,
}
