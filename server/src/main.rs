pub mod dice;
pub mod game;
pub mod load;
mod prelude;
pub mod script;
pub mod server;
mod shutdown;

use clap::Parser;
use load::try_load_toml;
use server::CommandServer;
use std::{path::PathBuf, sync::LazyLock, time::Duration};
use tokio::runtime;
use tracing::{info, Level};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

pub use prelude::*;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    #[arg(short, long, default_value = PathBuf::from("config.toml").into_os_string())]
    pub config: PathBuf,
}

static ARG: LazyLock<Args> = LazyLock::new(Args::parse);

fn main() -> anyhow::Result<()> {
    let filter = Targets::new()
        .with_default(Level::TRACE)
        .with_target("h2", Level::INFO);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("running with {:?}", *ARG);
    ctrlc::set_handler(|| trigger_shutdown())?;
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
    run.block_on(wait_shutdown());
    info!("shutting down in 30s");
    run.shutdown_timeout(Duration::from_secs(30));
    Ok(())
}
