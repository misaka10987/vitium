use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

/// Server configuration.
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub chat_cap: usize,
    pub module: Vec<String>,
}
impl ServerConfig {
    pub fn new(port: u16, chat_cap: usize, module: Vec<String>) -> Self {
        Self {
            port,
            chat_cap,
            module,
        }
    }
}

pub trait FromTOML<T>
where
    T: de::DeserializeOwned,
{
    fn obj(&self) -> T;
}

impl<T> FromTOML<T> for String
where
    T: de::DeserializeOwned,
{
    fn obj(&self) -> T {
        toml::from_str(self).expect("deserialize error")
    }
}

pub trait TOML
where
    Self: ser::Serialize + de::DeserializeOwned,
{
    fn toml(&self) -> String {
        toml::to_string(self).expect("serialize error")
    }
}

impl TOML for ServerConfig {}
