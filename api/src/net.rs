pub use crate::game::Action;
use crate::{game::PC, player::Player};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::SystemTime;

/// Describes a request.
/// Any type that implements `Req` should be correctly handled
/// if sent to the server with specified `PATH` and `METHOD`.
pub trait Req: Serialize + DeserializeOwned {
    /// The JSON body of response.
    type Response: Serialize + DeserializeOwned;
    /// The path this request should be sent to.
    fn path(&self) -> String;
    /// The method this request should be sent with.
    const METHOD: &'static str;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignUp {
    pub user: String,
    pub pass: String,
}

impl Req for SignUp {
    type Response = ();

    fn path(&self) -> String {
        "/api/auth/signup".into()
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enroll(pub String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Chat {
    /// The user who sends the message.
    pub sender: String,
    /// The chat message.
    pub msg: String,
    /// The time message is sent.
    pub send_time: SystemTime,
    /// The time message is received by server.
    pub recv_time: SystemTime,
}

impl Chat {
    pub fn new(sender: String, msg: String) -> Self {
        Self {
            sender,
            msg,
            send_time: SystemTime::now(),
            recv_time: SystemTime::UNIX_EPOCH,
        }
    }

    /// Receive the message, with current time as `.recv_time`.
    pub fn received(self) -> Chat {
        Chat {
            sender: self.sender,
            msg: self.msg,
            send_time: self.send_time,
            recv_time: SystemTime::now(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendChat(pub Chat);

impl Req for SendChat {
    type Response = SystemTime;

    fn path(&self) -> String {
        "/api/chat".into()
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecvChat(pub SystemTime);

impl Req for RecvChat {
    type Response = Vec<Chat>;

    fn path(&self) -> String {
        "/api/chat".into()
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListPlayer;

impl Req for ListPlayer {
    type Response = Vec<String>;

    fn path(&self) -> String {
        "/api/player".into()
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPlayer(#[serde(skip)] pub String);

impl Req for GetPlayer {
    type Response = Player;

    fn path(&self) -> String {
        format!("/api/player/{}", self.0)
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPlayer(#[serde(skip)] pub String, pub Player);

impl Req for EditPlayer {
    type Response = ();

    fn path(&self) -> String {
        format!("/api/player/{}", self.0)
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListPC;

impl Req for ListPC {
    type Response = Vec<(String, PC)>;

    fn path(&self) -> String {
        "/api/pc".into()
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPC(#[serde(skip)] pub String);

impl Req for GetPC {
    type Response = PC;

    fn path(&self) -> String {
        format!("/api/pc/{}", self.0)
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPC(#[serde(skip)] pub String, pub Option<PC>);

impl Req for EditPC {
    type Response = ();

    fn path(&self) -> String {
        format!("/api/pc/{}", self.0)
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPass(pub String);

impl Req for EditPass {
    type Response = ();

    fn path(&self) -> String {
        "/api/auth/pass".into()
    }

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize)]
pub struct Sync {}
