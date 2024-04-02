use serde::{Deserialize, Serialize};

/// Defines attribution of a Chara.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub base: i16,
    pub curr: i16,
}

impl Attr {
    pub fn new(base: i16) -> Self {
        Self { base, curr: base }
    }
    pub fn fix(&self, zero: i16, coef: i16) -> i16 {
        (self.curr - zero) / coef
    }
}
