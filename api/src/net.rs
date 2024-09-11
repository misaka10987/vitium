pub use crate::{cmd::Cmd, game::Action};
use crate::{
    game::PC,
    player::{Password, Player},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::SystemTime;

/// Describes a request.
/// Any type that implements `Req` should be correctly handled
/// if sent to the server with specified `PATH` and `METHOD`.
pub trait Req: Serialize + DeserializeOwned {
    /// The response expected if everything's ok.
    type Response: Serialize + DeserializeOwned;
    /// The path this request should be sent to.
    const PATH: &'static str;
    /// The method this request should be sent with.
    const METHOD: &'static str;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub password: Password,
    pub info: Player,
}

impl Req for CreatePlayer {
    type Response = ();

    const PATH: &'static str = "/api/create";

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enroll(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    /// The chat message.
    pub msg: String,
    /// The time message is sent.
    pub send_time: SystemTime,
    /// The time message is received by server.
    pub recv_time: SystemTime,
}

impl Chat {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            send_time: SystemTime::now(),
            recv_time: SystemTime::UNIX_EPOCH,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendChat(pub Chat);

impl Req for SendChat {
    type Response = SystemTime;

    const PATH: &'static str = "/api/chat";

    const METHOD: &'static str = "POST";
}

impl SendChat {
    /// Receive the message, with current time as `.recv_time`.
    pub fn received(self) -> Chat {
        let Self(Chat { msg, send_time, .. }) = self;
        Chat {
            msg,
            send_time,
            recv_time: SystemTime::now(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecvChat(pub SystemTime);

impl Req for RecvChat {
    type Response = Vec<(String, Chat)>;

    const PATH: &'static str = "/api/chat";

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPlayer();

impl Req for GetPlayer {
    type Response = Vec<(String, Player)>;

    const PATH: &'static str = "/api/player";

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPC();

impl Req for GetPC {
    type Response = Vec<(String, PC)>;

    const PATH: &'static str = "/api/pc";

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Edit<T, Id = String> {
    pub src: Id,
    pub dst: Option<T>,
}

impl Req for Edit<PC> {
    type Response = ();

    const PATH: &'static str = "/api/pc";

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPass(pub Password);

impl Req for EditPass {
    type Response = ();

    const PATH: &'static str = "/api/auth/pass";

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPlayer(pub Player);

impl Req for EditPlayer{
    type Response = ();

    const PATH: &'static str = "/api/player";

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize)]
pub struct Sync {}

pub type Res<T> = Result<<T as Req>::Response, String>;
