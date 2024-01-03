use std::cmp::{Eq, Ord};
use std::convert::From;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use serde_derive::{Deserialize, Serialize};

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

impl<T> Mul for Bottle<T>
where
    T: Mul<Output = T>,
{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        self.now.mul(rhs.now)
    }
}

impl<T> MulAssign for Bottle<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.now.mul_assign(rhs.now)
    }
}

impl<T> Div for Bottle<T>
where
    T: Div<Output = T>,
{
    type Output = T;
    fn div(self, rhs: Self) -> Self::Output {
        self.now.div(rhs.now)
    }
}

impl<T> DivAssign for Bottle<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.now.div_assign(rhs.now)
    }
}

// impl<T, U> Add<U> for Bottle<T>
// where
//     T: Add<U, Output = T>,
//     U: Into<T>,
// {
//     type Output = T;
//     fn add(self, rhs: U) -> Self::Output {
//         self.now.add(rhs)
//     }
// }
