use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::UId;

/// Error representing a UId with specific type currently does not exist.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NoUIdError<T>(UId<T>);

impl<T> Debug for NoUIdError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NoUIdError").field(&self.0).finish()
    }
}
