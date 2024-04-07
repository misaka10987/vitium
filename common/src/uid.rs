use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Hash, Serialize, Deserialize)]
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

impl<T> Clone for UID<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _t: PhantomData,
        }
    }
}

impl<T> Copy for UID<T> {}
