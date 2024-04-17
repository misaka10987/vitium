use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnimplError(pub String);

impl Display for UnimplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not implemented", self.0)
    }
}

impl Error for UnimplError {}

pub trait ErrorSer: Error + Serialize {}
