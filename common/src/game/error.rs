use std::{
    error::Error,
    fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

use crate::UId;

use super::TypeName;

/// Error representing a UId with specific type currently does not exist.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NoUIdError<T>(UId<T>);

impl<T: TypeName> Display for NoUIdError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} does not exist", self.0)
    }
}

impl<T> Debug for NoUIdError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NoUIdError").field(&self.0).finish()
    }
}

impl<T: TypeName> Error for NoUIdError<T> {}
