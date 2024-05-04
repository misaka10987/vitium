use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::Registry;

pub trait Data: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}

impl<T> Data for T where T: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}

pub struct Compon<R: Registry> {
    pub reg: Cow<'static, Box<R>>,
    pub data: R::Data,
}
