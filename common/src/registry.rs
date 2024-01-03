use std::collections::HashMap;

/// Defines constructor function.
pub type Maker<'a, T> = &'a dyn Fn() -> T;

/// Defines a registry item.
pub struct Regis<'a, T> {
    constructor: Maker<'a, T>,
}

impl<'a, T> Regis<'a, T> {
    pub fn new(constructor: Maker<'a, T>) -> Self {
        Self { constructor }
    }
    /// Generate a new instance.
    /// # Examples
    /// ```
    /// let f = || 114;
    /// let reg = Regis::new(&f);
    /// assert_eq!(reg.inst(),114);
    /// ```
    pub fn inst(&self) -> T {
        (self.constructor)()
    }
}

/// String ID for a registry.
pub type RegID = String;

/// Generic registry table using `HashMap`.
pub type RegTable<'a, T> = HashMap<RegID, Regis<'a, T>>;
