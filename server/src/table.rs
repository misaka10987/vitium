use std::{
    borrow::Cow,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use vitium_common::UID;

pub struct Table<'a, T, S>
where
    T: Clone + 'a,
    S: Query<'a, T>,
{
    map: HashMap<UID<T>, S>,
    _a: PhantomData<&'a ()>,
}

impl<'a, T, S> Deref for Table<'a, T, S>
where
    T: Clone + 'a,
    S: Query<'a, T>,
{
    type Target = HashMap<UID<T>, S>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<'a, T, S> DerefMut for Table<'a, T, S>
where
    T: Clone + 'a,
    S: Query<'a, T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub struct Section<'a, T: Clone>(HashMap<UID<T>, Cow<'a, T>>);

pub trait Query<'a, T>
where
    T: Clone + 'a,
{
    fn sec(
        &'a self,
    ) -> impl std::future::Future<Output = impl Deref<Target = Section<'a, T>>> + Send;
    fn sec_mut(
        &'a mut self,
    ) -> impl std::future::Future<Output = impl DerefMut<Target = Section<'a, T>>> + Send;
}
