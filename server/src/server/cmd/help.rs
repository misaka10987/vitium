use anyhow::bail;
use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

/// Print help page of command.
#[derive(Parser)]
#[command(name = "help")]
pub struct Help {
    /// The command to print help page.
    command: String,
}

impl Command for Help {
    async fn exec(self, s: Server) -> anyhow::Result<String> {
        match s.cmd.resolve(&self.command) {
            Some(cmd) => Ok(cmd.help_page()),
            None => bail!("help: unknown command '{}'", self.command),
        }
    }

    fn perm_req() -> Perm {
        Perm::from("")
    }
}
