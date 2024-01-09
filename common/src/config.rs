use serde_derive::{Deserialize, Serialize};

/// Server configuration.
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub chat_cap: usize,
    pub module: Vec<String>,
}
