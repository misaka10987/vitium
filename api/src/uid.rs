use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// A unique 64-bit integer id with type hint `T`.
///
/// Note that despite the generic parameter `T`,
/// this can be used to access component of any type if valid using `.to()`.
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct UId<T> {
    /// The id.
    pub value: u64,
    #[serde(skip)]
    _t: PhantomData<T>,
}

impl<T> UId<T> {
    /// Create new id.
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            _t: PhantomData,
        }
    }

    /// Convert to another component type.
    pub const fn to<U>(self) -> UId<U> {
        UId::new(self.value)
    }
}

impl<T> Deref for UId<T> {
    type Target = u64;

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

impl<T> Display for UId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "uid={:#x}", self.value)
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
