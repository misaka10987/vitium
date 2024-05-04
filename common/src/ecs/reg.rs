use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use super::{Data, Entity};

use crate::ID;

#[repr(transparent)]
pub struct RegTab<E: Entity>(HashMap<ID, E::Reg>);

impl<E: Entity> Deref for RegTab<E> {
    type Target = HashMap<ID, E::Reg>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E: Entity> DerefMut for RegTab<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E: Entity> RegTab<E> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Instantiate a value from a received pack.
    pub fn inst(&self, ox: Reg<E::Reg>) -> Option<Cow<E::Reg>> {
        match ox {
            Reg::ID(id) => self.get(&id).map(|r| Cow::Borrowed(r)),
            Reg::Custom(p) => Some(Cow::Owned(*p)),
        }
    }

    /// Pack up a value.
    /// # Panics
    /// Panic if `cow` is `Borrowed` from a registry item that does not specify an `ID`.
    pub fn save(&self, cow: Cow<E::Reg>) -> Reg<E::Reg> {
        match cow {
            Cow::Borrowed(r) => Reg::ID(r.as_ref().clone().unwrap()),
            Cow::Owned(t) => Reg::Custom(Box::new(t)),
        }
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    pub unsafe fn drop(reg: &'static Self) {
        drop(unsafe { Box::from_raw(reg as *const Self as *mut Self) });
    }
}

pub trait Registry: Clone + AsRef<Option<ID>> + 'static {
    type Data: Data;
}

pub trait Registered: ToOwned<Owned = Self> + AsRef<Option<ID>> + 'static {}

impl<T> Registered for T where T: ToOwned<Owned = Self> + AsRef<Option<ID>> + 'static {}

#[derive(Clone, Serialize, Deserialize)]
pub enum Reg<T> {
    ID(ID),
    Custom(Box<T>),
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
