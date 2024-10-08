use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::{DmgType, Target};

use super::Act;

#[derive(Clone, Serialize, Deserialize)]
pub struct Atk {
    pub target: Target,
}

impl Act for Atk {
    const SYNC: bool = true;

    type Success = HashMap<DmgType, i16>;

    type Failure = ();
}
