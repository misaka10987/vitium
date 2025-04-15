use std::{path::PathBuf, sync::LazyLock, time::Duration};

use clap::Parser;
use load::try_load_toml;
use server::CommandServer;
use tokio::{runtime, sync::broadcast};
use tracing::{info, Level};

/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
pub mod load;
pub mod script;
/// New server.
pub mod server;

pub use server::Server;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    #[arg(short, long, default_value = PathBuf::from("config.toml").into_os_string())]
    pub config: PathBuf,
}

static ARG: LazyLock<Args> = LazyLock::new(Args::parse);

static SHUTDOWN: LazyLock<broadcast::Sender<()>> = LazyLock::new(|| broadcast::channel(1).0);

fn shutdown() {
    let _ = SHUTDOWN.send(());
}

async fn recv_shutdown() {
    SHUTDOWN.subscribe().recv().await.unwrap()
}

fn main() -> anyhow::Result<()> {
    let filter = Targets::new()
        .with_default(Level::TRACE)
        .with_target("h2", Level::INFO);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("running with {:?}", *ARG);
    ctrlc::set_handler(|| shutdown())?;
    let run = runtime::Builder::new_multi_thread().enable_all().build()?;
    run.spawn(async {
        let cfg = try_load_toml(&ARG.config).await;
        let server = Server::new(cfg).await.unwrap();
        let input = server.handle_input();
        let output = server.print_cmd_output();
        server.start().await.unwrap();
        input.abort();
        output.abort();
    });
    run.block_on(recv_shutdown());
    info!("shutting down in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
