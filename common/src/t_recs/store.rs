use crate::UId;

use super::{reg::RegTab, Compon, ComponMut, Entity, Reg, Regis};

pub trait Store<E: Entity, T: Regis = <E as Entity>::Base>: AsRef<&'static RegTab<T>> {
    /// Get a component.
    fn compon(&self, idx: UId<E>) -> Option<Compon<T>>;
    /// Get a component as mutable.
    fn compon_mut(&mut self, idx: UId<E>) -> Option<ComponMut<T>>;
    /// Insert a component to the specified entity.
    fn ins_compon(&mut self, idx: UId<E>, compon: (Reg<T>, T::Data)) -> Option<(Reg<T>, T::Data)>;
    /// Remove a component from the specified entity.
    fn rm_compon(&mut self, idx: UId<E>) -> Option<(Reg<T>, T::Data)>;
}
