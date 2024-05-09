use serde::{de::Visitor, Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::game::TypeName;

/// A pointer-wide unique id for a specified type `T`.
///
/// This type is serialized into an unsigned 64bit integer using `serde`,
/// thus, on 32bit platforms, it is sliced during serialization,
/// with only the lower 32 bits reserved.
pub struct UId<T> {
    /// The UId.
    pub value: usize,
    _t: PhantomData<T>,
}

impl<T> UId<T> {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            _t: PhantomData,
        }
    }
}

impl<T> Deref for UId<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for UId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> PartialEq for UId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for UId<T> {}

impl<T> PartialOrd for UId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for UId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Clone for UId<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _t: PhantomData,
        }
    }
}

impl<T> Copy for UId<T> {}

impl<T> Hash for UId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: TypeName> Display for UId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>[uid={:#x}]", T::typename(), self.value)
    }
}

impl<T> Debug for UId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UId")
            .field("value", &self.value)
            .field("_t", &self._t)
            .finish()
    }
}

impl<T> Serialize for UId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.value as u64)
    }
}

struct UIdVisitor<T> {
    _t: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for UIdVisitor<T> {
    type Value = UId<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an unsigned pointer-wide (64bit) integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UId::new(v as usize))
    }
}

impl<'de, T> Deserialize<'de> for UId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(UIdVisitor::<T> { _t: PhantomData })
    }
}

#[cfg(test)]
mod test {
    use super::UId;
    use crate::{json, obj};

    #[test]
    fn serde() {
        let x = UId::<()>::new(114514);
        let y = json(&x).unwrap();
        let y = obj(&y).unwrap();
        assert_eq!(x, y);
    }
}
