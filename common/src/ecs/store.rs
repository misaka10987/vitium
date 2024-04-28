use crate::UID;

use super::{Compon, Entity};

/// Defines storage operation interface for a certain type of component.
///
/// # Note: `AsRef` reflexivity
///
///
pub trait Store<E, C = <E as Entity>::Base>
where
    E: Entity,
    C: Compon,
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
