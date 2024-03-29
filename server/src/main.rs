use std::process::exit;

use clap::Parser;
use server::Server;
use tokio::spawn;
use tracing::info;

/// In-game characters.
pub mod chara;
/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
/// Process the input when running server.
pub mod input;
/// Load game saves.
pub mod load;
/// Registry.
pub mod registry;
/// Save the game.
pub mod save;
/// In-game scenario.
pub mod scene;
pub mod script;
/// New server.
pub mod server;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the server configuration file.
    pub config: Option<String>,
}

#[tokio::main]
async fn main() {
    // initialize logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let arg = Args::parse();
    info!("Running with {:?}", arg);
    spawn(input::input());
    // run the server
    Server::new()
        .config("./config.toml")
        .run()
        .await
        .expect("internal server error");
    input::stop().await;
    exit(0)
}
