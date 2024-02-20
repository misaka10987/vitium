use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Feature {
    RegID(String),
    Script(String),
}
