use server::Server;

pub mod dice;
pub mod server;

pub const UNTIL: u64 = 500;

fn main() {
    Server::start()
        .set_port(19198)
        .run()
        .expect("An irrecoverable internal server error occured!")
}
