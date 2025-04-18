use basileus::Perm;
use clap::Parser;
use clearscreen::clear;

use crate::Server;

use super::Command;

/// Clear terminal screen.
#[derive(Parser)]
#[command(name = "clear", visible_alias = "cls")]
pub struct Clear;

impl Command for Clear {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        clear()?;
        Ok("".into())
    }

    fn perm_req() -> Perm {
        Perm::from("server-console")
    }
}
