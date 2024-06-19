pub mod atk;
pub mod walk;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::{atk::Atk, walk::Walk};

pub trait Act: Send + Serialize + DeserializeOwned {
    const SYNC: bool;
    type Res: Send + Serialize + DeserializeOwned;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Action {
    /// Request for synchronization.
    Sync,
    /// Request for asynchronization.
    Async,
    Atk(Atk),
    Walk(Walk),
}

macro_rules! from {
    ($t:ty,$f:ident) => {
        impl From<$t> for Action {
            fn from(value: $t) -> Self {
                Action::$f(value)
            }
        }
    };
}

from!(Atk, Atk);
from!(Walk, Walk);
