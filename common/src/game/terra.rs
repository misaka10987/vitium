use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Terra {
    /// Symbol displayed on the map.
    pub sym: char,
    /// Time used to pass, in APs.
    pub mov: i16,
    /// [0,1], 0 for completely transparently and 1 for completely opaque.
    pub opaque: f32,
}
