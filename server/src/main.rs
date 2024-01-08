use server::Server;

/// In-game characters.
pub mod chara;
/// Dice implementation using `ndm`.
pub mod dice;
/// Specific game logics goes here.
pub mod game;
/// Registry.
pub mod registry;
/// Defines the server.
pub mod server;

fn main() {
    Server::start()
        .set_port(19198)
        .run()
        .expect("internal server error")
}
