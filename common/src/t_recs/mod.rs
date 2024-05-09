pub mod compon;
pub mod prelude;
pub mod reg;
pub mod store;
pub mod uid;

pub use prelude::*;

pub trait Entity: 'static {
    type Base: Regis;
}
