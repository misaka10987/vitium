pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

pub trait Example
where
    Self: Sized,
{
    fn examples() -> Vec<Self>;
    fn example() -> Self {
        Self::examples().pop().unwrap()
    }
}

impl Example for () {
    fn examples() -> Vec<Self> {
        vec![(), ()]
    }
}

impl<T> Example for (T,)
where
    T: Example,
{
    fn examples() -> Vec<Self> {
        T::examples().into_iter().map(|x| (x,)).collect()
    }
}

macro_rules! impl_int {
    [$t:ty]=>{
        impl Example for $t {
            fn examples() -> Vec<Self> {
                vec![114]
            }
        }
    };
    [$t0:ty,$($t:ty),+] => {
        impl_int![$t0];
        impl_int![$($t),+];
    };
}

impl_int![i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
