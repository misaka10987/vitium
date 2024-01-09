use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

/// Server configuration.
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub chat_cap: usize,
    pub module: Vec<String>,
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
