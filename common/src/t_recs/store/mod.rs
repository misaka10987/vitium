pub mod btree;

use crate::UId;

use super::{Compon, Cr, Cw, Entity, Regis};

pub trait Store<E: Entity, T: Regis = <E as Entity>::Base>: BaseStore<E> {
    /// Get a component.
    fn compon(&self, idx: UId<E>) -> Option<Cr<T>>;
    /// Get a component as mutable.
    fn compon_mut(&mut self, idx: UId<E>) -> Option<Cw<T>>;
    /// Insert a component to the specified entity.
    fn ins_compon(&mut self, idx: UId<E>, compon: Compon<T>) -> Option<Compon<T>>;
    /// Remove a component from the specified entity.
    fn rm_compon(&mut self, idx: UId<E>) -> Option<Compon<T>>;
}

/// This is a marker to remind you to invoke `with_compon!`.
/// You should not implement it manually.
pub trait BaseStore<E: Entity> {}
