pub use crate::{cmd::Cmd, game::Action};
use crate::player::Password;
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
    /// Receive the message, with current time as `.recv_time`.
    pub fn received(self) -> Self {
        let Chat { msg, send_time, .. } = self;
        Self {
            msg,
            send_time,
            recv_time: SystemTime::now(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Edit<T, Id = String> {
    pub src: Id,
    pub dst: Option<T>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EditPswd(pub Password);
