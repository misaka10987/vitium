use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    reg::{RegReader, Regis},
    Reg,
};

pub trait Data: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {}

impl<T> Data for T where T: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {}

#[derive(Clone, Serialize, Deserialize)]
pub struct Compon<T: Regis>(pub Reg<T>, pub T::Data);

#[derive(Clone, Copy)]
pub struct Cr<'a, T: Regis> {
    pub reg: RegReader<'a, T>,
    pub data: &'a T::Data,
}

pub struct Cw<'a, T: Regis> {
    pub reg: RegReader<'a, T>,
    pub data: &'a mut T::Data,
}

#[cfg(test)]
mod test {
    use crate::{
        t_recs::{Reg, Regis},
        test::Example,
    };

    use super::Compon;

    impl<T: Regis> Example for Compon<T>
    where
        T: Example,
        T::Data: Example,
    {
        fn examples() -> Vec<Self> {
            vec![Self(
                Reg::Custom(Box::new(T::example())),
                T::Data::example(),
            )]
        }
    }
}
