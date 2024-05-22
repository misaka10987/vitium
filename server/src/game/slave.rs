pub mod walk;

use tokio::sync::Mutex;
use vitium_common::game::Scena;

pub struct Slave {
    pub scena: Scena,
    pub sync: bool,
}
