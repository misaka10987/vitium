use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

/// Output specified content.
#[derive(Parser)]
#[command(name = "echo")]
pub struct Echo {
    /// Content to output.
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
