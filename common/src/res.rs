use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{delta::Delta, game::Scena};

/// All possible responses are defined here.
#[derive(Serialize, Deserialize)]
pub enum Res{
    Sync(Sync),
}

#[derive(Serialize, Deserialize)]
pub struct Sync{}
