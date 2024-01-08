use std::collections::HashMap;
pub enum Regis<T> {
    Static(Box<dyn Fn() -> T>),
    Dynamic(T),
}

impl<T> Regis<T>
where
    T: Clone,
{
    /// Generate a new instance.
    /// # Examples
    /// ```
    /// use vitium_common::registry::Regis;
    /// let reg = Regis::Static(Box::new(|| 114));
    /// assert_eq!(reg.inst(), 114);
    /// let reg = Regis::Dynamic(514);
    /// assert_eq!(reg.inst(), 514);
    /// ```
    pub fn inst(&self) -> T {
        match self {
            Self::Static(f) => f(),
            Self::Dynamic(t) => t.clone(),
        }
    }
}

/// String ID for a registry.
pub type RegID = String;

/// Generic registry table using `HashMap`.
pub type RegTable<'a, T> = HashMap<RegID, Regis<T>>;
