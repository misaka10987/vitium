use std::process::exit;

use basileus::Perm;
use clap::Parser;

use crate::Server;

use super::Command;

/// Forcefully terminate the server.
#[derive(Parser)]
#[command(name = "kill")]
pub struct Kill;

impl Command for Kill {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        exit(-1)
    }

    fn perm_req() -> Perm {
        Perm::from("root")
    }
}
