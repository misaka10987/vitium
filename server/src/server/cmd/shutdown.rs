use basileus::Perm;
use clap::Parser;

use crate::{shutdown, Server};

use super::Command;

#[derive(Parser, Clone, Copy)]
#[command(name = "shutdown", visible_alias = "stop", visible_alias = "exit")]
#[clap(disable_help_flag = true)]
/// Shutdown the server with specified timeout.
pub struct Shutdown;

impl Command for Shutdown {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        shutdown();
        Ok("".into())
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
