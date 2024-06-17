pub mod compon;
pub mod prelude;
pub mod reg;
pub mod store;
pub mod uid;

pub use prelude::*;

pub trait Entity: 'static {
    type Base: Regis;
}

#[macro_export]
macro_rules! with_compon {
    [$self:ty,$e:ty,$($tx:ty),*] => {
        impl $crate::t_recs::store::BaseStore<$e> for $self {}
        impl $self {
            pub fn rm_entity(&mut self, idx: $crate::t_recs::UId<$e>) {
                use $crate::t_recs::Store;
                $crate::rm_entity![self, idx, $($tx), *];
            }
        }
    };
}

#[macro_export]
/// Never call this directly.
macro_rules! rm_entity {
    [$self:expr,$idx:expr,$t:ty] => {
        let _: std::option::Option<$crate::t_recs::Compon<$t>> = $self.rm_compon($idx);
    };
    [$self:expr,$idx:expr,$t:ty,$($tx:ty),+]=>{
        let _: std::option::Option<$crate::t_recs::Compon<$t>> = $self.rm_compon($idx);
        $crate::rm_entity![$self,$idx,$($tx),+];
    };
}
