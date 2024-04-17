pub use crate::{cmd::Cmd, game::Act};
use crate::{game::PC, player::Player};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[cfg(test)]
use crate::test::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Enroll {
    pub chara: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub msg: String,
    pub time: SystemTime,
}

impl Chat {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            time: SystemTime::now(),
        }
    }
    pub fn renew(&mut self) -> &mut Self {
        self.time = SystemTime::now();
        self
    }
}

pub type EditPlayer = Player;

#[derive(Serialize, Deserialize, Clone)]
pub struct EditChara {
    pub dest: String,
    pub new: PC<'static>,
}

#[cfg(test)]
impl Example for EditChara {
    fn examples() -> Vec<Self> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendChat {
    pub chat: Chat,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPswd {
    pub pswd: String,
}

impl EditPswd {
    pub fn new() -> Self {
        Self {
            pswd: "debug-pswd".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exit {
    pub chara: i128,
}

/// All possible requests are defined here.
#[derive(Serialize, Deserialize)]
pub enum Req<'a> {
    /// Get current server status.
    ServerStatus,
    /// Synchronize all available data.
    Sync(String),
    /// Receive out-game chat messages.
    RecvChat,
    /// Synchronize player list.
    GetPlayer,
    /// Synchronize character list.
    GetChara,
    /// Send out-game chat message.
    SendChat(SendChat),
    /// Create, edit or delete player.
    EditPlayer(EditPlayer),
    /// Create, edit or delete character.
    EditChara(EditChara),
    /// Change password.
    EditPswd(EditPswd),
    /// Submit in-game action.
    Act(Act<'a>),
    /// Issue server command.
    Cmd(Cmd),
}

impl<'a> Req<'a> {
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
