use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::UID;

pub type Delta<T> = BTreeMap<UID<T>, Option<T>>;

/// All possible responses are defined here.
#[derive(Serialize, Deserialize)]
pub enum Res {}
