use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[derive(Clone, Serialize, Deserialize)]
pub struct Attr;

def_regtab!(Attr, R_ATTR);
