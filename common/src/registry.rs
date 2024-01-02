/// Registry
pub struct Regis<'a, T> {
    pub id: String,
    constructor: &'a dyn Fn() -> T,
}

impl<'a, T> Regis<'a, T> {
    pub fn new(id: String, constructor: &dyn Fn() -> T) -> Regis<T> {
        Regis { id, constructor }
    }

    /// Creates a new `T` instance using internal constructor.
    /// # Examples
    /// ```
    /// let f = || 114;
    /// let reg = Regis::new("test".to_string(), &f);
    /// assert_eq!(reg.inst(),114);
    /// ```
    pub fn inst(&self) -> T {
        (self.constructor)()
    }
}
