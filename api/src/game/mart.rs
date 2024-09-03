use serde::{Deserialize, Serialize};

use crate::def_regtab;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mart;

def_regtab!(Mart, R_MART);
