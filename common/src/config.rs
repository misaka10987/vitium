use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

/// Server configuration.
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub chat_cap: usize,
    pub module: Vec<String>,
}
impl ServerConfig {
    pub fn new() -> Self {
        Self {
            port: 54321,
            chat_cap: 255,
            module: vec![],
        }
    }
}

pub fn toml<T>(obj: &T) -> String
where
    T: ser::Serialize,
{
    toml::to_string(&obj).expect("serialize error")
}

pub fn obj<T>(toml: &str) -> T
where
    T: de::DeserializeOwned,
{
    toml::from_str(toml).expect("deserialize error")
}

