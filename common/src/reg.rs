use std::collections::HashMap;

use crate::ID;

/// Generic registry table using `HashMap`.
pub type Reg<T> = HashMap<ID, T>;

#[macro_export]
macro_rules! gen_reg {
    ($t:ty) => {
        use crate::reg::Reg;
        use once_cell::sync::Lazy;

        static mut REG: Lazy<Reg<$t>> = Lazy::new(|| HashMap::new());

        /// Getter for the registry table.
        ///
        /// Be careful for the risk of concurrent modification.
        pub fn reg() -> &'static Reg<$t> {
            unsafe { &REG }
        }

        /// Getter for the registry table.
        ///
        /// Be careful for the risk of concurrent modification.
        pub fn reg_mut() -> &'static mut Reg<$t> {
            unsafe { &mut REG }
        }
    };
}
