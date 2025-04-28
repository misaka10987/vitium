mod cli;
mod crash;
mod prelude;
mod script;
mod server;

use clap::Parser;
use crash::{crash, crashed};
use std::{fs::read_to_string, panic, path::PathBuf, time::Duration};
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

fn main() -> Result<(), ()> {
    let args = Vitium::parse();
    panic::set_hook(Box::new(crash));
    ctrlc::set_handler(|| shutup::ROOT.shut()).expect("register signal hook");
    let mut builder = runtime::Builder::new_multi_thread();
    builder.enable_all();
    let run = builder.build().expect("runtime setup");
    let cfg = read_to_string(&args.config).expect("load config");
    let cfg = toml::from_str(&cfg).expect("parse config");
    let server = run.block_on(Server::new(cfg)).expect("initialize");
    cli::start(server.clone()).expect("start command REPL");
    let shutdown = run.block_on(server.start()).expect("server start");
    run.block_on(shutdown.wait());
    info!("shutdown in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    if crashed() { Err(()) } else { Ok(()) }
}
