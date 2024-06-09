use std::{
    collections::HashMap,
    error::Error,
    fmt::{write, Display},
    fs,
    ops::{Deref, DerefMut},
    path::Path,
};

use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Serialize,
};
use tracing::trace;

use super::Data;

/// A type that can be registered.
pub trait Regis: 'static {
    /// The mutating data which this registery constraints.
    type Data: Data;
}

#[macro_export]
/// Automatically generate `impl Regis` for the specified type.
/// `type Data=();` if no `Data` type is specified.
macro_rules! regis {
    ($r:ty) => {
        regis!($r, ());
    };
    ($r:ty,$d:ty) => {
        impl $crate::t_recs::reg::Regis for $r {
            type Data = $d;
        }
    };
}

/// Representing registry information for a specific type `T`.
#[derive(Clone, Hash, Serialize, Deserialize)]
pub enum Reg<T> {
    /// Representing an already registered `Id`.
    Id(Id),
    /// Representing custom registries.
    Custom(Box<T>),
}

/// A smart pointer that allows reading registry information
/// regardless of whether it has been registered.
#[derive(Clone, Copy)]
pub struct RegReader<'a, 'b, T: Regis> {
    tab: &'a RegTab<T>,
    reg: &'b Reg<T>,
}

impl<T: Regis> Deref for RegReader<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.reg {
            Reg::Id(id) => self.tab.get(id).unwrap(),
            Reg::Custom(b) => b,
        }
    }
}

#[repr(transparent)]
pub struct RegTab<T: Regis>(pub HashMap<Id, T>);

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
    /// Creates an empty instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Reads the referenced `Reg`, returns a smart pointer.
    pub fn read<'a, 'b>(&'a self, maybe_reg: &'b Reg<T>) -> RegReader<'a, 'b, T> {
        RegReader {
            tab: self,
            reg: maybe_reg,
        }
    }

    /// Merges another `RegTab` into `self`, returns an iterator for items overridden.
    pub fn merge(&mut self, other: Self) -> impl Iterator<Item = (Id, T)> + '_ {
        let RegTab(map) = other;
        map.into_iter()
            .filter_map(|(k, v)| self.insert(k.to_owned(), v).map(|v| (k, v)))
    }

    /// Leaks this `RegTab` to be `'static`, used when loading completed.
    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    /// Unsafely drops the `RegTab`, used when the game is no longer needed.
    /// Be careful to make sure that all its references have already been dropped.
    pub unsafe fn drop(reg: &'static Self) {
        drop(Box::from_raw(reg as *const Self as *mut Self));
    }
}

impl<T> RegTab<T>
where
    T: Regis + DeserializeOwned,
{
    /// Loads the registry table from every `.json` file in the specified directory.
    ///
    /// Note: it is **underined behaviour** to have multiple registries with the same id.
    pub fn load(p: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut tab = RegTab::<T>(HashMap::new());
        for i in fs::read_dir(p)? {
            let i = i?;
            if i.metadata()?.is_file() {
                if let Some(filename) = i.file_name().to_str() {
                    if filename.ends_with(".json") {
                        trace!("loading \"{}\"", filename);
                        let s = fs::read_to_string(i.path())?;
                        let part: HashMap<_, _> = serde_json::from_str(&s)?;
                        tab.extend(part);
                    }
                }
            }
        }
        Ok(tab)
    }

    /// `load` registry table from directory and `.merge` it to `self`
    pub fn load_more(
        &mut self,
        p: impl AsRef<Path>,
    ) -> Result<impl Iterator<Item = (Id, T)> + '_, Box<dyn Error>> {
        Ok(self.merge(Self::load(p)?))
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

#[macro_export]
macro_rules! with_reg {
    ($t:ty,$f:ident,$c:ty) => {
        impl std::convert::AsRef<&'static $crate::t_recs::reg::RegTab<$c>> for $t {
            fn as_ref(&self) -> &&'static $crate::t_recs::reg::RegTab<$c> {
                &self.$f
            }
        }
    };
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
