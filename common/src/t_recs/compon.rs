use serde::{Deserialize, Serialize};

use super::reg::{RegReader, Regis};

pub trait Data: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}

impl<T> Data for T where T: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}

#[derive(Clone, Copy)]
pub struct Compon<'a, T: Regis> {
    pub reg: RegReader<'a, T>,
    pub data: &'a T::Data,
}

pub struct ComponMut<'a, T: Regis> {
    pub reg: RegReader<'a, T>,
    pub data: &'a mut T::Data,
}
