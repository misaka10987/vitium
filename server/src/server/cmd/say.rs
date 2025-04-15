use basileus::Perm;
use clap::Parser;

use crate::{server::chat::ChatServer, Server};

use super::Command;

#[derive(Parser)]
#[command(name = "say")]
#[clap(disable_help_flag = true)]
/// Broadcast a server message.
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
