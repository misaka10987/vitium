use server::Server;

/// In-game characters.
pub mod chara;
/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
/// Load game saves.
pub mod load;
/// Registry.
pub mod registry;
/// Save the game.
pub mod save;
/// Defines the server.
pub mod server;

fn main() {
    Server::start()
        .set_port(19198)
        .run()
        .expect("internal server error")
}
