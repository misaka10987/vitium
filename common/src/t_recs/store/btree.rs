use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    delta::{Delta, DeltaList, PackDeltaList},
    t_recs::{reg::RegTab, Compon, ComponReader, ComponWriter, Entity, Regis},
    UId,
};

use super::Store;

pub struct BTreeStore<E: Entity, T: Regis = <E as Entity>::Base> {
    map: BTreeMap<UId<E>, Compon<T>>,
    delta: DeltaList<UId<E>, Option<Compon<T>>>,
}

impl<E: Entity, C: Regis, T> Store<E, C> for T
where
    T: AsRef<BTreeStore<E, C>> + AsMut<BTreeStore<E, C>> + AsRef<&'static RegTab<C>>,
{
    fn compon(&self, idx: UId<E>) -> Option<crate::t_recs::ComponReader<C>> {
        let rt: &RegTab<C> = *self.as_ref();
        let store: &BTreeStore<E, C> = self.as_ref();
        if let Some(Compon(reg, data)) = store.map.get(&idx) {
            let reg = rt.read(reg);
            Some(ComponReader { reg, data })
        } else {
            None
        }
    }

    fn compon_mut(&mut self, idx: UId<E>) -> Option<crate::t_recs::ComponWriter<C>> {
        let rt: &RegTab<C> = *self.as_ref();
        let store: &mut BTreeStore<E, C> = self.as_mut();
        if let Some(Compon(reg, data)) = store.map.get_mut(&idx) {
            let reg = rt.read(reg);
            Some(ComponWriter { reg, data })
        } else {
            None
        }
    }

    fn ins_compon(&mut self, idx: UId<E>, compon: Compon<C>) -> Option<Compon<C>> {
        let store: &mut BTreeStore<E, C> = self.as_mut();
        store.map.insert(idx, compon)
    }

    fn rm_compon(&mut self, idx: UId<E>) -> Option<Compon<C>> {
        let store: &mut BTreeStore<E, C> = self.as_mut();
        store.map.remove(&idx)
    }
}

impl<E, T> Delta for BTreeStore<E, T>
where
    E: Entity,
    T: Serialize + Deserialize<'static> + Regis,
{
    type Pack = PackDeltaList<UId<E>, Option<Compon<T>>>;

    fn calc(&mut self) -> &Self::Pack {
        self.delta.pack()
    }

    fn diff(&self) -> &Self::Pack {
        self.delta.pack()
    }

    fn apply(&mut self, delta: Self::Pack) {
        for (k, v) in delta {
            match v {
                Some(c) => self.map.insert(k, c),
                None => self.map.remove(&k),
            };
        }
    }
}

#[macro_export]
macro_rules! with_btree_store {
    ($t:ty,$f:ident,$e:ty) => {
        with_btree_store!($t, $f, $e, <$e as $crate::t_recs::Entity>::Base);
    };
    ($t:ty,$f:ident,$e:ty,$c:ty) => {
        impl std::convert::AsRef<$crate::t_recs::store::btree::BTreeStore<$e, $c>> for $t {
            fn as_ref(&self) -> &$crate::t_recs::store::btree::BTreeStore<$e, $c> {
                &self.$f
            }
        }

        impl std::convert::AsMut<$crate::t_recs::store::btree::BTreeStore<$e, $c>> for $t {
            fn as_mut(&mut self) -> &mut $crate::t_recs::store::btree::BTreeStore<$e, $c> {
                &mut self.$f
            }
        }
    };
}
