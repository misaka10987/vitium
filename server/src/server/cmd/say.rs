use basileus::Perm;
use clap::Parser;

use crate::Server;

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
        s.send_server_chat(self.message).await;
        Ok("".into())
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
