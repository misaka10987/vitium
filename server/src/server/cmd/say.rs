use basileus::Perm;
use clap::Parser;

use crate::{server::chat::ChatServer, Server};

use super::Command;

/// Broadcast a server message.
#[derive(Parser)]
#[command(name = "say", visible_alias = "broadcast")]
pub struct Say {
    /// The message.
    message: String,
}

impl Command for Say {
    async fn exec(self, s: Server) -> anyhow::Result<String> {
        s.send_server_msg(self.message).await;
        Ok("".into())
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
