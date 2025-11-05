use basileus::Perm;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::filter::Targets;

use crate::Server;

use super::Command;

/// Set the log verbosity level.
#[derive(Parser)]
#[command(name = "loglevel")]
pub struct LogLevel {
    /// The level to set to.
    level: Level,
    /// If specified, would apply changes only to the module.
    #[arg(short, long = "mod")]
    module: Option<String>,
}

impl Command for LogLevel {
    async fn exec(self, s: Server) -> anyhow::Result<String> {
        let setter = |filter: &mut Targets| match self.module {
            Some(m) => *filter = filter.clone().with_target(m, self.level),
            None => *filter = filter.clone().with_default(self.level),
        };
        s.update_logger(setter)?;
        Ok(format!("set log level to {}", self.level))
    }

    fn perm_req() -> Perm {
        Perm::from("admin")
    }
}
