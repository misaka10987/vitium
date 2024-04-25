use tokio::sync::RwLock;
use vitium_common::game::Scena;

pub struct Slave<'a> {
    pub scena: RwLock<Scena<'a>>,
}
