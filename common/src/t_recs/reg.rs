use std::{
    collections::HashMap,
    fmt::{write, Display},
    ops::{Deref, DerefMut},
};

use serde::{de::Visitor, Deserialize, Serialize};

use super::Data;

pub trait Regis: 'static {
    type Data: Data;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Reg<T> {
    Id(Id),
    Custom(Box<T>),
}

/// A smart pointer that allows reading registry information
/// regardless of whether it has been registered.
#[derive(Clone, Copy)]
pub struct RegReader<'a, T: Regis> {
    tab: &'static RegTab<T>,
    reg: &'a Reg<T>,
}

impl<T: Regis> Deref for RegReader<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.reg {
            Reg::Id(id) => self.tab.get(id).unwrap(),
            Reg::Custom(b) => b,
        }
    }
}

#[repr(transparent)]
pub struct RegTab<T: Regis>(HashMap<Id, T>);

impl<T: Regis> Deref for RegTab<T> {
    type Target = HashMap<Id, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Regis> DerefMut for RegTab<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Regis> RegTab<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn read<'a>(&'static self, maybe_reg: &'a Reg<T>) -> RegReader<'a, T> {
        RegReader {
            tab: self,
            reg: maybe_reg,
        }
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    pub unsafe fn drop(reg: &'static Self) {
        drop(unsafe { Box::from_raw(reg as *const Self as *mut Self) });
    }
}

/// String identifier for a class of entity, usually used in registeries.
///
/// # Formats
///
/// A valid `Id` should contain one and only one char `:` that seperates it into two strings,
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
/// An `Id` with an empty module name (like `:builtin-id`) is for the vitium-builtin objects.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Id {
    /// The local identifier, should be unique inside the module,
    /// even if the type is different.
    pub id: String,
    /// Module name.
    pub module: String,
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}:{}", self.id, self.module))
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.id, self.module))
    }
}

struct IdVisitor;

impl<'de> Visitor<'de> for IdVisitor {
    type Value = Id;

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
        Ok(Id::new(s[0], s[1]))
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor)
    }
}

impl Id {
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
mod test {
    use super::Id;
    use crate::test::*;
    impl Example for Id {
        fn examples() -> Vec<Self> {
            vec![Id::new("example-id", "example-module")]
        }
    }
}
