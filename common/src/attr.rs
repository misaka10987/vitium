use crate::{bottle::Bottle, envelop::Envelop};
use serde_derive::{Deserialize, Serialize};

pub type TpAttr = u16;

/// Defines attribution of a character.
#[derive(Serialize, Deserialize, Clone)]
pub struct Attr {
    pub id: String,
    pub value: Envelop<Bottle<TpAttr>>,
}

impl Attr {
    pub fn new(id: String, value: Envelop<Bottle<TpAttr>>) -> Self {
        Self { id, value }
    }
}
