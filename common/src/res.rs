use serde::{Deserialize, Serialize};

/// All possible responses are defined here.
#[derive(Serialize, Deserialize)]
pub enum Res {
    Sync(Sync),
}

#[derive(Serialize, Deserialize)]
pub struct Sync {}
