pub mod btree;

use crate::UId;

use super::{Compon, ComponReader, ComponWriter, Entity, Regis};

pub trait Store<E: Entity, T: Regis = <E as Entity>::Base> {
    /// Get a component.
    fn compon(&self, idx: UId<E>) -> Option<ComponReader<T>>;
    /// Get a component as mutable.
    fn compon_mut(&mut self, idx: UId<E>) -> Option<ComponWriter<T>>;
    /// Insert a component to the specified entity.
    fn ins_compon(&mut self, idx: UId<E>, compon: Compon<T>) -> Option<Compon<T>>;
    /// Remove a component from the specified entity.
    fn rm_compon(&mut self, idx: UId<E>) -> Option<Compon<T>>;
}
