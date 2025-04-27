mod cli;
mod dice;
mod prelude;
mod script;
mod server;

use clap::Parser;
use std::{fs::read_to_string, path::PathBuf, time::Duration};
use tokio::runtime;
use tracing::info;

pub use prelude::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Vitium {
    /// Path to the server configuration file.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Vitium::parse();
    ctrlc::set_handler(|| shutup::ROOT.shut())?;
    let run = runtime::Builder::new_multi_thread().enable_all().build()?;
    let cfg = if let Some(path) = &args.config {
        toml::from_str(&read_to_string(path)?)?
    } else {
        Default::default()
    };
    let server = run.block_on(Server::new(cfg))?;
    cli::start(server.clone())?;
    let shutdown = run.block_on(server.start())?;
    run.block_on(shutdown.wait());
    info!("shutdown in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
