use serde::{Deserialize, Serialize};

use crate::def_regtab;

#[derive(Serialize, Deserialize, Clone)]
pub struct Terra {
    /// Symbol displayed on the map.
    pub sym: char,
    /// Time used to pass, in APs. `None` for terrains that block movement.
    pub mov: Option<i16>,
    /// [0,1], 0 for completely transparently and 1 for completely opaque.
    pub opaque: f32,
}

def_regtab!(Terra, R_TERRA);
