use serde_derive::{Deserialize, Serialize};
use std::cmp::{Eq, Ord};
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// An envelop is used to hide certain members of a struct.
#[derive(Serialize, Deserialize)]
pub enum Envelop<T> {
    Open(T),
    Closed,
}

impl<T> Clone for Envelop<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Open(arg0) => Self::Open(arg0.clone()),
            Self::Closed => Self::Closed,
        }
    }
}
impl<T> From<T> for Envelop<T> {
    fn from(value: T) -> Self {
        Self::Open(value)
    }
}

/// Container for values with maximum.
#[derive(Serialize, Deserialize, Clone)]
pub struct Bottle<T> {
    pub now: T,
    pub cap: T,
}

impl<T> Bottle<T>
where
    T: From<u8>,
{
    pub fn min() -> T {
        T::from(0)
    }
    pub fn new(cap: T) -> Bottle<T> {
        Bottle::<T> {
            now: Bottle::min(),
            cap,
        }
    }
}

impl<T> PartialEq for Bottle<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.now == other.now
    }
}

impl<T> Eq for Bottle<T>
where
    Bottle<T>: PartialEq,
    T: Eq,
{
}

impl<T> PartialOrd for Bottle<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.now.partial_cmp(&other.now)
    }
}

impl<T> Ord for Bottle<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.now.cmp(&other.now)
    }
}

impl<T> Add for Bottle<T>
where
    T: Add<Output = T>,
{
    type Output = T;
    fn add(self, rhs: Self) -> Self::Output {
        self.now.add(rhs.now)
    }
}

impl<T> AddAssign for Bottle<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.now.add_assign(rhs.now)
    }
}

impl<T> Sub for Bottle<T>
where
    T: Sub<Output = T>,
{
    type Output = T;
    fn sub(self, rhs: Self) -> Self::Output {
        self.now.sub(rhs.now)
    }
}

impl<T> SubAssign for Bottle<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.now.sub_assign(rhs.now)
    }
}
