use std::{error::Error, path::Path};

use clap::Parser;
use tracing::{info, Level};

use crate::{input::input, server::ServerConfig};

/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
// pub mod game;
/// Process the input when running server.
pub mod input;
pub mod script;
/// New server.
pub mod server;

pub use server::Server;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    pub config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let arg = Args::parse();
    info!("running with {:?}", arg);
    let cfg = ServerConfig::try_load(Path::new("./config.toml")).await;
    let input = input();
    Server::with_cfg(cfg).run().await?;
    input.send(()).expect("failed to shutdown input thread");
    Ok(())
}
