use std::{
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

use serde::{de::Visitor, Deserialize, Serialize};

use crate::game::TypeName;

pub struct UID<T> {
    pub value: usize,
    _t: PhantomData<T>,
}

impl<T> UID<T> {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            _t: PhantomData,
        }
    }
}

impl<T> PartialEq for UID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for UID<T> {}

impl<T> PartialOrd for UID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for UID<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Clone for UID<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _t: PhantomData,
        }
    }
}

impl<T> Copy for UID<T> {}

impl<T> Hash for UID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: TypeName> Display for UID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[uid={:#x}]", T::typename(), self.value)
    }
}

impl<T> Debug for UID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UID")
            .field("value", &self.value)
            .field("_t", &self._t)
            .finish()
    }
}

impl<T> Serialize for UID<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.value as u64)
    }
}

struct UIDVisitor<T> {
    _t: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for UIDVisitor<T> {
    type Value = UID<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an unsigned pointer-wide (64bit) integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UID::new(v.try_into().unwrap()))
    }
}

impl<'de, T> Deserialize<'de> for UID<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(UIDVisitor::<T> { _t: PhantomData })
    }
}

#[cfg(test)]
mod test {
    use crate::{json, obj, UID};

    #[test]
    fn serde() {
        let x = UID::<()>::new(114514);
        let y = json(&x).unwrap();
        let y = obj(&y).unwrap();
        assert_eq!(x, y);
    }
}
