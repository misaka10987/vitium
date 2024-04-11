use std::{hash::Hash, marker::PhantomData};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UID<T> {
    pub value: u64,
    _t: PhantomData<T>,
}

impl<T> UID<T> {
    pub fn new(value: u64) -> Self {
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
