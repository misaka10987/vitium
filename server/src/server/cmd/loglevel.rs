use basileus::Perm;
use clap::Parser;
use tracing::Level;

use crate::{log::FILTER, Server};

use super::Command;

/// Set the log verbosity level.
#[derive(Parser)]
#[command(name = "loglevel")]
#[clap(disable_help_flag = true)]
pub struct LogLevel {
    /// The level to set to.
    level: Level,
    /// If specified, would apply changes only to the module.
    #[arg(short, long = "mod")]
    module: Option<String>,
}

impl Command for LogLevel {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        FILTER.lock().unwrap().modify(|filter| match self.module {
            Some(m) => *filter = filter.clone().with_target(m, self.level),
            None => *filter = filter.clone().with_default(self.level),
        })?;
        Ok(format!("set log level to {}", self.level))
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
