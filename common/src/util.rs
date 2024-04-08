use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord};
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::ID;

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

/// This is a utility that reminds you whether some data should be available.
pub struct Envelop<T> {
    inside: T,
}

impl<T> Envelop<T> {
    pub fn new(inside: T) -> Envelop<T> {
        Envelop { inside }
    }
    pub fn expose(self) -> T {
        self.inside
    }
    pub fn expose_if(self, condition: bool) -> Option<T> {
        if condition {
            Some(self.inside)
        } else {
            None
        }
    }
    pub fn unseal(&self) -> &T {
        &self.inside
    }
    pub fn unseal_if(&self, condition: bool) -> Option<&T> {
        if condition {
            Some(&self.inside)
        } else {
            None
        }
    }
}

impl<T> Clone for Envelop<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inside: self.inside.clone(),
        }
    }
}

impl<T> Copy for Envelop<T> where T: Clone + Copy {}

pub enum Ox<'a, T> {
    Reg(&'a ID),
    Inst(&'a T),
}

impl<T: Clone> Clone for Ox<'_, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Reg(x) => Self::Reg(x),
            Self::Inst(x) => Self::Inst(x),
        }
    }
}

impl<T: Clone> Copy for Ox<'_, T> {}
