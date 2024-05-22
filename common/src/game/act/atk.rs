use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::{DmgType, Obj};

use super::Act;

#[derive(Clone, Serialize, Deserialize)]
pub struct Atk {
    pub target: Obj,
}

impl Act for Atk {
    const SYNC: bool = true;

    type Res = AtkRes;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AtkRes {
    Success(HashMap<DmgType, i16>),
    Miss,
}
