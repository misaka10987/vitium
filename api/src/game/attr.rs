use serde::{Deserialize, Serialize};

use crate::def_regtab;

#[derive(Clone, Serialize, Deserialize)]
pub struct Attr;

def_regtab!(Attr, R_ATTR);
