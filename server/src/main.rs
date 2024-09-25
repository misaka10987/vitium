use std::path::PathBuf;

use clap::Parser;
use load::try_load_toml;
use tokio::runtime;
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

fn main() -> anyhow::Result<()> {
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            tracing_subscriber::fmt()
                .with_max_level(Level::TRACE)
                .init();
            let arg = Args::parse();
            info!("running with {:?}", arg);
            let cfg = try_load_toml(arg.config).await;
            let server = Server::new(cfg).await;
            let input = server.input();
            server.run().await?;
            input.send(()).expect("failed to shutdown input thread");
            Ok(())
        })
}
