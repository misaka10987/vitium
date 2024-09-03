use serde::{Deserialize, Serialize};

use crate::req::Req;

#[derive(Serialize, Deserialize)]
pub struct Sync {}

pub type Res<T> = Result<<T as Req>::Response, String>;
