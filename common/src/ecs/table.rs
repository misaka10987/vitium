use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::UID;

use super::{Compon, Entity, Store};

pub struct BTreeTable<E, C = <E as Entity>::Base>
where
    E: Entity,
    C: Compon,
{
    map: BTreeMap<UID<E>, C>,
}

impl<E: Entity, C: Compon> Deref for BTreeTable<E, C> {
    type Target = BTreeMap<UID<E>, C>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<E: Entity, C: Compon> DerefMut for BTreeTable<E, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<E: Entity, C: Compon> Store<E, C> for BTreeTable<E, C> {
    fn compon(&self, idx: UID<E>) -> Option<&C> {
        self.get(&idx)
    }

    fn compon_mut(&mut self, idx: UID<E>) -> Option<&mut C> {
        self.get_mut(&idx)
    }

    fn ins_compon(&mut self, idx: UID<E>, compon: C) -> Option<C> {
        self.insert(idx, compon)
    }

    fn rm_compon(&mut self, idx: UID<E>) -> Option<C> {
        self.remove(&idx)
    }
}

#[macro_export]
macro_rules! with_table {
    ($t:ty,$f:ident,$e:ty,$c:ty) => {
        impl $crate::ecs::Store<$e, $c> for $t {
            fn compon(&self, idx: $crate::ecs::UID<$e>) -> std::option::Option<&$c> {
                self.$f.compon(idx)
            }

            fn compon_mut(&mut self, idx: $crate::ecs::UID<$e>) -> std::option::Option<&mut $c> {
                self.$f.compon_mut(idx)
            }

            fn ins_compon(
                &mut self,
                idx: $crate::ecs::UID<$e>,
                compon: $c,
            ) -> std::option::Option<$c> {
                self.$f.ins_compon(idx, compon)
            }

            fn rm_compon(&mut self, idx: $crate::ecs::UID<$e>) -> std::option::Option<$c> {
                self.$f.rm_compon(idx)
            }
        }
    };
}
