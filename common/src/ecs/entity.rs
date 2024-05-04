use super::{reg::Registered, Data};

pub trait Entity: 'static {
    type Reg: Registered;
    type Base: Data;
}
