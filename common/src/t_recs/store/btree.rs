use std::collections::BTreeMap;

use crate::{
    t_recs::{reg::RegTab, Compon, ComponReader, ComponWriter, Entity, Regis},
    UId,
};

use super::Store;

pub struct BtreeStore<E: Entity, T: Regis = <E as Entity>::Base> {
    map: BTreeMap<UId<E>, Compon<T>>,
}

impl<E: Entity, C: Regis, T> Store<E, C> for T
where
    T: AsRef<BtreeStore<E, C>> + AsMut<BtreeStore<E, C>> + AsRef<&'static RegTab<C>>,
{
    fn compon(&self, idx: UId<E>) -> Option<crate::t_recs::ComponReader<C>> {
        let rt: &RegTab<C> = *self.as_ref();
        let store: &BtreeStore<E, C> = self.as_ref();
        if let Some(Compon(reg, data)) = store.map.get(&idx) {
            let reg = rt.read(reg);
            Some(ComponReader { reg, data })
        } else {
            None
        }
    }

    fn compon_mut(&mut self, idx: UId<E>) -> Option<crate::t_recs::ComponWriter<C>> {
        let rt: &RegTab<C> = *self.as_ref();
        let store: &mut BtreeStore<E, C> = self.as_mut();
        if let Some(Compon(reg, data)) = store.map.get_mut(&idx) {
            let reg = rt.read(reg);
            Some(ComponWriter { reg, data })
        } else {
            None
        }
    }

    fn ins_compon(&mut self, idx: UId<E>, compon: Compon<C>) -> Option<Compon<C>> {
        let store: &mut BtreeStore<E, C> = self.as_mut();
        store.map.insert(idx, compon)
    }

    fn rm_compon(&mut self, idx: UId<E>) -> Option<Compon<C>> {
        let store: &mut BtreeStore<E, C> = self.as_mut();
        store.map.remove(&idx)
    }
}

struct TestEntity;

impl Entity for TestEntity {
    type Base = TestComponReader;
}

struct TestComponReader;

impl Regis for TestComponReader {
    type Data = ();
}

#[macro_export]
macro_rules! btree_store {
    ($t:ty,$f:ident,$e:ty,$c:ty) => {
        impl std::convert::AsRef<$crate::t_recs::store::btree::BtreeStore<$e, $c>> for $t {
            fn as_ref(&self) -> &$crate::t_recs::store::btree::BtreeStore<$e, $c> {
                &self.$f
            }
        }

        impl std::convert::AsMut<$crate::t_recs::store::btree::BtreeStore<$e, $c>> for $t {
            fn as_mut(&mut self) -> &mut $crate::t_recs::store::btree::BtreeStore<$e, $c> {
                &mut self.$f
            }
        }
    };
}
