use serde::{Deserialize, Serialize};

use super::Act;

#[derive(Clone, Serialize, Deserialize)]
pub struct Walk {
    pub speed: f32,
    pub mov: i16,
    pub dest: (f32, f32),
}

impl Act for Walk {
    const SYNC: bool = false;

    type Success = ();

    type Failure = ();
}
