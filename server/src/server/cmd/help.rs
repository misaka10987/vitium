use anyhow::bail;
use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

#[derive(Parser)]
#[command(name = "help")]
#[command(about = "print help page of command")]
#[clap(disable_help_flag = true)]
pub struct Help {
    command: String,
    #[arg(short, long)]
    x: Option<String>,
}

impl Command for Help {
    async fn exec(self, s: Server) -> anyhow::Result<String> {
        let cmd = match s.cmd.resolve(&self.command) {
            Some(x) => x,
            None => bail!("help: unknown command '{}'", self.command),
        };
        let mut clap = cmd.clap.clone();

        Ok(format!("{}", clap.render_long_help().ansi()))
    }

    fn perm_req() -> Perm {
        Perm::from("")
    }
}
