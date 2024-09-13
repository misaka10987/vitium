pub mod walk;

use vitium_api::game::Scena;

pub struct Slave {
    pub scena: Scena,
    pub sync: bool,
}
