use serde_derive::{Deserialize, Serialize};

/// An envelop is used to hide certain members of a struct.
#[derive(Serialize, Deserialize)]
pub enum Envelop<T> {
    Open(T),
    Closed,
}

impl<T> Clone for Envelop<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Open(arg0) => Self::Open(arg0.clone()),
            Self::Closed => Self::Closed,
        }
    }
}

impl<T> From<T> for Envelop<T> {
    fn from(value: T) -> Self {
        Self::Open(value)
    }
}
