use std::process::exit;

use clap::Parser;
use server::Server;
use tracing::info;

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

fn main() {
    // initialize logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let arg = Args::parse();
    info!("Running with {:?}", arg);
    // run the server
    Server::new()
        .config("./config.toml")
        .run()
        .expect("internal server error");
    exit(0)
}
