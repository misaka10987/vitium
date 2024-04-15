use serde::{Deserialize, Serialize};

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

#[test]
fn see_json() {
    use crate::json;
    let c = Cmd {
        cmd: Command::Hello,
    };
    println!("{}", json(&c).unwrap());
}
