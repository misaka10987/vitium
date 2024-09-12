use serde::{Deserialize, Serialize};

use fe3o4::def_regtab;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mart;

def_regtab!(Mart, R_MART);
