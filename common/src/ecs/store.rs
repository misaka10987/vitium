use crate::UId as UID;

use super::{Data, Entity};

/// Defines storage operation interface for a certain type of component.
pub trait Store<E, C = <E as Entity>::Base>
where
    E: Entity,
    C: Data,
{
    /// Get a component.
    fn compon(&self, idx: UID<E>) -> Option<&C>;
    /// Get a component as mutable.
    fn compon_mut(&mut self, idx: UID<E>) -> Option<&mut C>;
    /// Insert a component to the specified entity.
    fn ins_compon(&mut self, idx: UID<E>, compon: C) -> Option<C>;
    /// Remove a component from the specified entity.
    fn rm_compon(&mut self, idx: UID<E>) -> Option<C>;
}
