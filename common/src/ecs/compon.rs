use serde::{Deserialize, Serialize};

pub trait Compon: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}

impl<T> Compon for T where T: Clone + Send + Sync + Serialize + Deserialize<'static> + 'static {}
