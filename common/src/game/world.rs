use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::UID;

use super::Scena;

pub struct World<'a> {
    pub scena: HashMap<UID<Scena<'a>>, RwLock<Scena<'a>>>,
}
