use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt::{write, Display};

#[cfg(test)]
use crate::test::*;

/// String identifier for a class of entity, usually used in registeries.
///
/// # Formats
///
/// A valid `ID` should contain one and only one char `:` that seperates it into two strings,
/// with the former stands for module name and latter stands for id, e.g. `example-module:example-id`.
///
/// Despite the fact that case-sensitive characters,
/// some special characters and Unicode *is* supported,
/// it is strongly suggestted **NOT** using such characters.
///
/// The ASCII characters `a`-`z`, `0`-`9` and `-._+` are recommandded.
///
/// # Special rules
///
/// An `ID` with an empty module name (like `:builtin-id`) is for the vitium-builtin objects.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ID {
    /// The local identifier, should be unique inside the module,
    /// even if the type is different.
    pub id: String,
    /// Module name.
    pub module: String,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}:{}", self.id, self.module))
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.id, self.module))
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
            return Err(E::missing_field("`id` or `module`"));
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
    pub fn new(id: &str, module: &str) -> Self {
        Self {
            module: module.to_string(),
            id: id.to_string(),
        }
    }
    pub fn builtin(id: &str) -> Self {
        Self::new(id, "")
    }
}

#[cfg(test)]
impl Example for ID {
    fn examples() -> Vec<Self> {
        vec![ID::new("example-id", "example-module")]
    }
}
