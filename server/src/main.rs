mod cli;
mod crash;
mod dice;
mod prelude;
mod script;
mod server;

use clap::Parser;
use crash::{crash, exit};
use std::{
    fs::read_to_string,
    panic::{self},
    path::PathBuf,
    time::Duration,
};
use tokio::runtime;
use tracing::info;

pub use prelude::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Vitium {
    /// Path to the server configuration file.
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Vitium::parse();
    panic::set_hook(Box::new(crash));
    ctrlc::set_handler(|| shutup::ROOT.shut())?;
    let run = runtime::Builder::new_multi_thread().enable_all().build()?;
    let cfg = toml::from_str(&read_to_string(&args.config).expect("failed to load config"))
        .expect("failed to parse config");
    let server = run.block_on(Server::new(cfg))?;
    cli::start(server.clone())?;
    let shutdown = run.block_on(server.start())?;
    run.block_on(shutdown.wait());
    info!("shutdown in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    exit()
}
