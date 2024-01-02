use serde_derive::{Deserialize, Serialize};

use crate::envelop::Envelop;

#[derive(Serialize, Deserialize)]
pub struct Prof {
    pub id:String,
    pub name:String,
    pub credit:Envelop<u16>,
}