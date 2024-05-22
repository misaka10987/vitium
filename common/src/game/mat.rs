use crate::{regis, typename};

use super::DmgType;
use std::collections::HashMap;

pub struct Mat {
    /// In g/cm^3.
    pub density: f32,
    /// Per-milimetre resistance to damage.
    pub resist: HashMap<DmgType, f32>,
}
typename!(Mat, "Material");
regis!(Mat);
