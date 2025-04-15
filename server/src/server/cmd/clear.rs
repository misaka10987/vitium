use basileus::Perm;
use clap::Parser;
use clearscreen::clear;

use crate::Server;

use super::Command;

#[derive(Parser, Clone, Copy)]
#[command(name = "clear", visible_alias = "cls")]
#[command(about = "clear screen")]
#[clap(disable_help_flag = true)]
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
