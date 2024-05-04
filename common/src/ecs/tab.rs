// use std::{
//     collections::BTreeMap,
//     ops::{Deref, DerefMut},
// };

// use crate::UID;

// use super::{Data, Entity, Reg, Store};

// pub struct BTreeTab<E, C = <E as Entity>::Base>
// where
//     E: Entity,
//     C: Compon,
// {
//     map: BTreeMap<UID<E>, C>,
// }

// pub type BTreeTabReg<E> = BTreeTab<E, Reg<<E as Entity>::Reg>>;

// impl<E: Entity, C: Compon> Deref for BTreeTab<E, C> {
//     type Target = BTreeMap<UID<E>, C>;

//     fn deref(&self) -> &Self::Target {
//         &self.map
//     }
// }

// impl<E: Entity, C: Compon> DerefMut for BTreeTab<E, C> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.map
//     }
// }

// impl<T, E: Entity, C: Compon> Store<E, C> for T
// where
//     T: AsRef<BTreeTab<E, C>> + AsMut<BTreeTab<E, C>>,
// {
//     fn compon(&self, idx: UID<E>) -> Option<&C> {
//         self.as_ref().get(&idx)
//     }

//     fn compon_mut(&mut self, idx: UID<E>) -> Option<&mut C> {
//         self.as_mut().get_mut(&idx)
//     }

//     fn ins_compon(&mut self, idx: UID<E>, compon: C) -> Option<C> {
//         self.as_mut().insert(idx, compon)
//     }

//     fn rm_compon(&mut self, idx: UID<E>) -> Option<C> {
//         self.as_mut().remove(&idx)
//     }
// }
