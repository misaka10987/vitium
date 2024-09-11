use serde::{Deserialize, Serialize};

use crate::def_regtab;

use super::DmgType;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Mat {
    /// In g/cm^3.
    pub density: f32,
    /// Per-milimetre resistance to damage.
    pub resist: HashMap<DmgType, f32>,
}

def_regtab!(Mat, R_MAT);
