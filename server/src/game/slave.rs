use tokio::sync::RwLock;
use vitium_common::game::Scena;

pub struct Slave {
    pub scena: RwLock<Scena>,
}
