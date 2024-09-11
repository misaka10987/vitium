pub mod id;
pub mod prelude;
pub mod tab;

use std::ops::Deref;

use dashmap::mapref::one::Ref;
pub use prelude::*;
use serde::{de::DeserializeOwned, Serialize};

pub trait Register: Sync + Serialize + DeserializeOwned + 'static {}
impl<T> Register for T where T: Sync + Serialize + DeserializeOwned + 'static {}

pub enum Rp<T: Register> {
    Registered(Ref<'static, &'static str, T>),
    Orphan(Box<T>),
}

impl<T: HasRegTab> Deref for Rp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Rp::Registered(p) => p,
            Rp::Orphan(p) => p,
        }
    }
}
