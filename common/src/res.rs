use serde::{Deserialize, Serialize};

use crate::{delta::Delta, game::Scena};

/// All possible responses are defined here.
#[derive(Serialize, Deserialize)]
pub enum Res<'a> {
    Sync(Sync<'a>),
}

#[derive(Serialize, Deserialize)]
pub struct Sync<'a> {
    pub dscena: Vec<<Scena<'a> as Delta>::Pack>,
}
