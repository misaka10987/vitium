use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

#[derive(Parser)]
#[command(name = "echo")]
#[command(about = "print command input to output")]
#[clap(disable_help_flag = true)]
pub struct Echo {
    input: String,
}

impl Command for Echo {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        Ok(self.input)
    }

    fn perm_req() -> Perm {
        Perm::from("")
    }
}
