use std::{error::Error, fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericError(pub String);

impl Deref for GenericError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GenericError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnimplError(pub String);

impl Deref for UnimplError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for UnimplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not implemented", self.0)
    }
}

impl Error for UnimplError {}

pub trait ErrorSer: Error + Serialize {}
