use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt::{write, Display};

#[cfg(test)]
use crate::test::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ID {
    pub module: String,
    pub id: String,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}:{}", self.module, self.id))
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.module, self.id))
    }
}

struct IDVisitor;

impl<'de> Visitor<'de> for IDVisitor {
    type Value = ID;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string that can be splitted by a ':' into two valid identifiers"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let s: Vec<&str> = v.split(':').collect();
        if s.len() != 2 {
            return Err(E::missing_field("id"));
        }
        Ok(ID::new(s[0], s[1]))
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IDVisitor)
    }
}

impl ID {
    pub fn new(module: &str, id: &str) -> Self {
        Self {
            module: module.to_string(),
            id: id.to_string(),
        }
    }
}

#[cfg(test)]
impl Example for ID {
    fn examples() -> Vec<Self> {
        vec![ID::new("example-module", "example-id")]
    }
}
