use crate::{json::JSON, player::Token};
use serde_derive::{Deserialize, Serialize};

/// An internal command processed by the game.
#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    Hello,
    Grant(String),
}
impl Command {
    pub fn op(&self) -> bool {
        match self {
            Command::Hello => false,
            Command::Grant(_) => true,
        }
    }
}

/// Command with authentication infomation.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cmd {
    pub cmd: Command,
    pub token: Token,
}
impl JSON for Cmd {}

/// Command echo.
#[derive(Serialize, Deserialize, Clone)]
pub struct Echo {
    /// Returning value of a command.
    pub value: i8,
    /// Output of a command.
    pub output: String,
}

#[test]
fn see_json() {
    let c = Cmd {
        cmd: Command::Hello,
        token: Token::new("114", "514"),
    };
    println!("{}", c.json());
}
