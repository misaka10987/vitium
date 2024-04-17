use std::{
    error::Error,
    fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

use crate::UID;

use super::TypeName;

/// Error representing a UID with specific type currently does not exist.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NoUIDError<T>(UID<T>);

impl<T: TypeName> Display for NoUIDError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} does not exist", self.0)
    }
}

impl<T> Debug for NoUIDError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NoUIDError").field(&self.0).finish()
    }
}

impl<T: TypeName> Error for NoUIDError<T> {}
