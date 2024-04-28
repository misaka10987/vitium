use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::ID;

pub struct RegTable<T: AsRef<Option<ID>>>(HashMap<ID, T>);

impl<T> Deref for RegTable<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    type Target = HashMap<ID, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RegTable<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> RegTable<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Instantiate a value from a received pack.
    pub fn inst(&self, ox: Reg<T>) -> Option<Cow<T>> {
        match ox {
            Reg::ID(id) => self.get(&id).map(|r| Cow::Borrowed(r)),
            Reg::Custom(p) => Some(Cow::Owned(*p)),
        }
    }

    /// Pack up a value.
    /// # Panics
    /// Panic if `cow` is `Borrowed` from a registry item that does not specify an `ID`.
    pub fn save(&self, cow: &Cow<T>) -> Reg<T> {
        match cow {
            Cow::Borrowed(r) => Reg::ID(r.as_ref().clone().unwrap()),
            Cow::Owned(t) => Reg::Custom(Box::new(t.clone())),
        }
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    pub unsafe fn drop(reg: &'static Self) {
        drop(unsafe { Box::from_raw(reg as *const Self as *mut Self) });
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Reg<T> {
    ID(ID),
    Custom(Box<T>),
}

