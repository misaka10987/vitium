use std::{path::PathBuf, time::Duration};

use clap::Parser;
use load::try_load_toml;
use once_cell::sync::Lazy;
use tokio::{runtime, sync::broadcast};
use tracing::{info, Level};

/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
/// Process the input when running server.
pub mod input;
pub mod load;
pub mod script;
/// New server.
pub mod server;

pub use server::Server;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    #[arg(short, long, default_value = PathBuf::from("config.toml").into_os_string())]
    pub config: PathBuf,
}

static ARG: Lazy<Args> = Lazy::new(Args::parse);

static SHUTDOWN: Lazy<broadcast::Sender<()>> = Lazy::new(|| broadcast::channel(1).0);

fn shutdown() {
    let _ = SHUTDOWN.send(());
}

async fn recv_shutdown() {
    SHUTDOWN.subscribe().recv().await.unwrap()
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    info!("running with {:?}", ARG);
    ctrlc::set_handler(|| shutdown())?;
    let run = runtime::Builder::new_multi_thread().enable_all().build()?;
    run.spawn(async {
        let cfg = try_load_toml(&ARG.config).await;
        let server = Server::new(cfg).await;
        let input = server.input();
        server.run().await.unwrap();
        input.abort();
    });
    run.block_on(recv_shutdown());
    info!("shutting down in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
