pub use crate::{act::Act, cmd::Cmd, player::Token};
use crate::{chara::Chara, player::Player, DEBUG_MSG};
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
    pub fn new() -> Self {
        Self {
            msg: DEBUG_MSG.to_string(),
            player: "debug-player".to_string(),
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

impl EditPlayer {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            token: Some(Token::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditChara {
    pub chara: Chara,
    pub token: Token,
}

impl EditChara {
    pub fn new() -> Self {
        Self {
            chara: Chara::new(),
            token: Token::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendChat {
    pub chat: Chat,
    pub token: Token,
}

impl SendChat {
    pub fn new() -> Self {
        Self {
            chat: Chat::new(),
            token: Token::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPswd {
    pub token: Token,
    pub pswd: String,
}

impl EditPswd {
    pub fn new() -> Self {
        Self {
            token: Token::new(),
            pswd: "debug-pswd".to_string(),
        }
    }
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
    /// Send out-game chat message.
    SendChat(Token),
    /// Create, edit or delete player.
    EditPlayer(EditPlayer),
    /// Create, edit or delete character.
    EditChara(EditChara),
    /// Change password.
    EditPswd(EditPswd),
    /// Submit in-game action.
    Act(Act),
    /// Issue server command.
    Cmd(Cmd),
}

impl Req {
    pub fn route(&self) -> &'static str {
        match self {
            Req::ServerStatus => "GET /",
            Req::Sync(_) => "GET /sync",
            Req::RecvChat => "GET /chat",
            Req::GetPlayer => "GET /player",
            Req::GetChara => "GET /chara",
            Req::SendChat(_) => "POST /chat",
            Req::EditPlayer(_) => "POST player",
            Req::EditChara(_) => "POST chara",
            Req::Act(_) => "POST /act",
            Req::Cmd(_) => "POST /cmd",
            Req::EditPswd(_) => "POST /pswd",
        }
    }
}

#[test]
fn seejson() {
    use crate::{chara::Chara, player::Token};
    use serde_json::to_string as json;
    macro_rules! see_json {
        ($d:expr,$v:expr) => {
            println!("{}", json(&$d).unwrap());
            println!("{}", json(&$v).unwrap());
        };
        ($t:ty) => {
            let v = <$t>::new();
            println!("{}", json(&v).unwrap());
        };
    }
    see_json!(Chara);
    see_json!(Token);
    see_json!(SendChat);
    see_json!(EditPswd);
    see_json!(EditPlayer);
    see_json!(EditChara);
}
