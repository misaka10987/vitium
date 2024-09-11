pub mod atk;
pub mod walk;

use std::ops::{Deref, DerefMut};

use atk::Atk;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use walk::Walk;

pub trait Act: Serialize + DeserializeOwned {
    const SYNC: bool;
    type Success: Serialize + DeserializeOwned;
    type Failure: Serialize + DeserializeOwned;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Action<T> {
    pub pc: String,
    pub action: T,
}

impl<T> Deref for Action<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.action
    }
}

impl<T> DerefMut for Action<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.action
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Error {
    Interrupted(String),
    Synchronicity(String),
    Invalid(String),
    Unimplemented,
}

pub type Result<T> =
    std::result::Result<std::result::Result<<T as Act>::Success, <T as Act>::Failure>, Error>;

macro_rules! req_act {
    ($act:ty,$name:expr) => {
        impl $crate::net::Req for $crate::game::act::Action<$act> {
            type Response = $crate::game::act::Result<$act>;

            const PATH: &'static str = concat!("/api/act/", $name);

            const METHOD: &'static str = "POST";
        }
    };
}

req_act!(Atk, "attack");
req_act!(Walk, "walk");
