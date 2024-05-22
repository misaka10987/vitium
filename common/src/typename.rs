pub trait TypeName {
    const TYPENAME: &'static str;
}

#[macro_export]
macro_rules! typename {
    ($t:ty,$s:expr) => {
        impl $crate::typename::TypeName for $t {
            const TYPENAME: &'static str = $s;
        }
    };
}