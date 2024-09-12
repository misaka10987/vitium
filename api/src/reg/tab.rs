use std::ops::{Deref, DerefMut};

use dashmap::DashMap;

use super::Register;

pub struct RegTab<T: Register>(pub(crate) DashMap<&'static str, T>);

pub trait HasRegTab: Register {
    fn reg_rab() -> &'static RegTab<Self>;
}

impl<T: Register> Deref for RegTab<T> {
    type Target = DashMap<&'static str, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Register> DerefMut for RegTab<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub use dashmap;
pub use lazy_static;

#[macro_export]
macro_rules! def_regtab {
    ($t:ty,$i:ident) => {
        $crate::reg::tab::lazy_static::lazy_static! {
            pub static ref $i: $crate::RegTab<$t> = $crate::RegTab($crate::reg::tab::dashmap::DashMap::new());
        }
        impl $crate::HasRegTab for $t {
            fn reg_rab() -> &'static $crate::RegTab<Self> {
                &$i
            }
        }
    };
}
