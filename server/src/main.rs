use std::{error::Error, path::PathBuf};

use clap::Parser;
use load::try_load_toml;
use tracing::{info, Level};

use crate::input::input;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let arg = Args::parse();
    info!("running with {:?}", arg);
    let cfg = try_load_toml(arg.config).await;
    let input = input();
    Server::new(cfg).await.run().await?;
    input.send(()).expect("failed to shutdown input thread");
    Ok(())
}
