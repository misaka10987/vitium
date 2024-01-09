use crate::player::Token;
use serde_derive::{Deserialize, Serialize};

/// An internal command processed by the game.
#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    Hello,
}

/// Command with authentication infomation.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cmd {
    pub cmd: Command,
    pub token: Token,
}

#[test]
fn see_json() {
    let c = Cmd {
        cmd: Command::Hello,
        token: Token::new("114", "514"),
    };
    println!("{}", crate::json::json(c));
}
