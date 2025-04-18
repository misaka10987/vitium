pub mod dice;
pub mod game;
pub mod load;
mod log;
mod prelude;
pub mod script;
pub mod server;

use clap::Parser;
use load::try_load_toml;
use shutup::ShutUp;
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
    let shutdown = ShutUp::new();
    let fut = shutdown.wait();
    run.spawn(async move {
        let cfg = try_load_toml(&ARG.config).await;
        let server = Server::new(cfg).await.unwrap();
        server.start().await.unwrap().adopt(&shutdown);
    });
    run.block_on(fut);
    info!("shutdown in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
