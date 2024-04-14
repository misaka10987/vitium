use std::error::Error;

use clap::Parser;
use server::Server;
use tracing::{info, Level};

/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
/// Process the input when running server.
pub mod input;
pub mod script;
/// New server.
pub mod server;
pub mod table;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    pub config: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // initialize logger
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let arg = Args::parse();
    info!("running with {:?}", arg);
    // run the server
    Server::new().config("./config.toml").run()?;
    Ok(())
}
