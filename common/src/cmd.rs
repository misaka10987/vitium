use serde::{Deserialize, Serialize};

pub type Echo = Result<String, String>;

/// An internal command processed by the game.
#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    Hello,
    Grant(String),
    ShutDown,
}
impl Command {
    pub fn op(&self) -> bool {
        match self {
            Command::Hello => false,
            Command::Grant(_) => true,
            Command::ShutDown => true,
        }
    }
}

/// Command with authentication infomation.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cmd {
    pub cmd: Command,
}
