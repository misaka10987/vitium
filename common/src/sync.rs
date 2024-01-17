use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sync {
    pub meta: String,
}

impl Sync {
    pub fn new() -> Self {
        Self {
            meta: "Not Implemented".to_string(),
        }
    }
}
