use serde::{Deserialize, Serialize};

use crate::{
    t_recs::{reg::Builtin, Regis},
    Id,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Terra {
    /// Symbol displayed on the map.
    pub sym: char,
    /// Time used to pass, in APs. `None` for terrains that block movement.
    pub mov: Option<i16>,
    /// [0,1], 0 for completely transparently and 1 for completely opaque.
    pub opaque: f32,
}

impl Regis for Terra {
    type Data = ();
}

#[rustfmt::skip]
impl Builtin for Terra {
    fn builtin() -> impl IntoIterator<Item = (Id, Self)> {[
        (Id::builtin("void"), Terra { sym: '🚫', mov: None, opaque: 0.0, },),
        (Id::builtin("barrier"), Terra { sym: '⛔', mov: None, opaque: 1.0, },),    
    ]}
}
