use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::ID;

pub struct Reg<T: AsRef<Option<ID>>>(HashMap<ID, T>);

impl<T> Deref for Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    type Target = HashMap<ID, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Reg<T>
where
    T: Clone + AsRef<Option<ID>>,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Instantiate a value from a received pack.
    pub fn inst(&self, ox: Ox<T>) -> Option<Cow<T>> {
        match ox {
            Ox::Reg(id) => self.get(&id).map(|r| Cow::Borrowed(r)),
            Ox::Inst(p) => Some(Cow::Owned(*p)),
        }
    }

    /// Pack up a value.
    /// # Panics
    /// Panic if `cow` is `Borrowed` from a registry item that does not specify an `ID`.
    pub fn save(&self, cow: &Cow<T>) -> Ox<T> {
        match cow {
            Cow::Borrowed(r) => Ox::Reg(r.as_ref().clone().unwrap()),
            Cow::Owned(t) => Ox::Inst(Box::new(t.clone())),
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
pub enum Ox<T> {
    Reg(ID),
    Inst(Box<T>),
}

#[macro_export]
macro_rules! impl_reg {
    ($t:ty) => {
        impl_reg!($t, reg);
    };
    ($t:ty,$f:ident) => {
        impl std::convert::AsRef<std::option::Option<$crate::ID>> for $t {
            fn as_ref(&self) -> &std::option::Option<$crate::ID> {
                &self.$f
            }
        }
    };
}
