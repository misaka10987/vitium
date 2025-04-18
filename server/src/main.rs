mod cli;
mod dice;
mod load;
mod log;
mod prelude;
mod script;
mod server;

use clap::Parser;
use load::try_load_toml;
use std::{path::PathBuf, sync::LazyLock, time::Duration};
use tokio::runtime;
use tracing::info;

pub use prelude::*;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    #[arg(short, long, default_value = PathBuf::from("config.toml").into_os_string())]
    pub config: PathBuf,
}

static ARG: LazyLock<Args> = LazyLock::new(Args::parse);

fn main() -> anyhow::Result<()> {
    info!("running with {:?}", *ARG);
    ctrlc::set_handler(|| shutup::ROOT.shut())?;
    let run = runtime::Builder::new_multi_thread().enable_all().build()?;
    let cfg = run.block_on(try_load_toml(&ARG.config));
    let server = run.block_on(Server::new(cfg))?;
    cli::start(server.clone())?;
    let shutdown = run.block_on(server.start())?;
    run.block_on(shutdown.wait());
    info!("shutdown in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
