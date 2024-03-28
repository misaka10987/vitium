use crate::DmgType;
use std::collections::HashMap;

pub struct Material {
    /// In g/cm^3.
    pub density: f32,
    pub resist: HashMap<DmgType, f32>,
}
