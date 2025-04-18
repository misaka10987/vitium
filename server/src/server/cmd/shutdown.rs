use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

/// Shutdown the server with specified timeout.
#[derive(Parser)]
#[command(name = "shutdown", visible_alias = "stop", visible_alias = "exit")]
#[clap(disable_help_flag = true)]
pub struct Shutdown;

impl Command for Shutdown {
    async fn exec(self, s: Server) -> anyhow::Result<String> {
        s.shutdown.shut();
        Ok("".into())
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
