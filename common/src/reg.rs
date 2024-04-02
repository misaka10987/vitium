use std::collections::HashMap;

use crate::ID;

/// Generic registry table using `HashMap`.
pub type Reg<T> = HashMap<ID, T>;
