pub use crate::{act::Act, cmd::Cmd, player::Token};
use crate::{
    chara::Chara,
    player::Player,
};
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Enroll {
    pub chara: Chara,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub msg: String,
    pub player: String,
    pub time: SystemTime,
}
impl Chat {
    pub fn new(msg: &str, player: &str) -> Self {
        Self {
            msg: msg.to_string(),
            player: player.to_string(),
            time: SystemTime::now(),
        }
    }
    pub fn renew(&mut self) -> &mut Self {
        self.time = SystemTime::now();
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPlayer {
    pub player: Player,
    pub token: Option<Token>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditChara {
    pub chara: Chara,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendChat {
    pub chat: Chat,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPswd {
    pub token: Token,
    pub pswd: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exit {
    pub chara: i128,
    pub token: Token,
}

/// All possible requests are defined here.
#[derive(Serialize, Deserialize)]
pub enum Req {
    /// Get current server status.
    ServerStatus,
    /// Synchronize all available data.
    Sync(Token),
    /// Receive out-game chat messages.
    RecvChat,
    /// Synchronize player list.
    GetPlayer,
    /// Synchronize character list.
    GetChara,
    /// Change password.
    EditPswd(EditPswd),
    /// Enroll in the game.
    Enroll(Enroll),
    /// Send out-game chat message.
    SendChat(Token),
    /// Create, edit or delete player.
    EditPlayer(EditPlayer),
    /// Create, edit or delete character.
    EditChara(EditChara),
    /// Submit in-game action.
    Act(Act),
    /// Issue server command.
    Cmd(Cmd),
    /// Exit the game temporarily.
    Exit,
}

impl Req {
    pub fn route(&self) -> &'static str {
        match self {
            Req::ServerStatus => "GET /",
            Req::Sync(_) => "GET /sync",
            Req::RecvChat => "GET /chat",
            Req::GetPlayer => "GET /player",
            Req::GetChara => "GET /chara",
            Req::Enroll(_) => "POST /enroll",
            Req::SendChat(_) => "POST /chat",
            Req::EditPlayer(_) => "POST player",
            Req::EditChara(_) => "POST chara",
            Req::Act(_) => "POST /act",
            Req::Cmd(_) => "POST /cmd",
            Req::EditPswd(_) => "POST /pswd",
            Req::Exit => "POST /exit",
        }
    }
}

#[test]
fn see_json() {
    use crate::{chara::Chara, player::Token};
    use serde_json::to_string as json;
    let t = Token::new("example_player", "example_password");
    let c = Chara::new();
    println!("{}", json(&Req::ServerStatus).unwrap());
    println!("{}", json(&Req::Sync(t.clone())).unwrap());
    println!("{}", json(&Req::RecvChat).unwrap());
    println!("{}", json(&Req::GetPlayer).unwrap());
    println!("{}", json(&Req::GetChara).unwrap());
    println!(
        "{}",
        json(&Req::Enroll(Enroll {
            chara: c.clone(),
            token: t.clone()
        }))
        .unwrap()
    );
    println!("{}", json(&Req::SendChat(t.clone())).unwrap());
    //println!("{}", json(&Req::EditPlayer(p, Some(t.clone()))).unwrap());
    //println!("{}", json(&Req::EditChara(c.clone(), t.clone())).unwrap());
    // println!(
    //     "{}",
    //     json(&Req::Act("Say hello world.".to_string(), t.clone())).unwrap()
    // );
    // println!(
    //     "{}",
    //     json(&Req::Cmd("help --help".to_string(), t.clone())).unwrap()
    // )
}
