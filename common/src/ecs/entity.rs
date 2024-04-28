use super::Compon;

pub trait Entity {
    type Reg;
    type Base: Compon;
}
