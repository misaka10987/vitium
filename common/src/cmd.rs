use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Echo(pub Result<String, String>);

impl Deref for Echo {
    type Target = Result<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Echo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
