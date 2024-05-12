use serde::{Deserialize, Serialize};

use crate::{regis, typename};

#[derive(Serialize, Deserialize, Clone)]
pub struct Mart {}
typename!(Mart, "MartialArt");
regis!(Mart);
