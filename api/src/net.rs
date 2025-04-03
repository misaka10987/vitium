pub use crate::game::Action;
use crate::{game::PC, user::User};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Describes a request.
/// Any type that implements `Req` should be correctly handled
/// if sent to the server with specified `PATH` and `METHOD`.
pub trait Req: Serialize + DeserializeOwned {
    /// The JSON body of response.
    type Response: Serialize + DeserializeOwned;
    /// The path this request should be sent to.
    fn path(&self) -> String;
    /// The method this request should be sent with.
    const METHOD: &'static str;
}

pub trait REST {

}

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SignUp {
    pub user: String,
    pub pass: String,
}

impl Req for SignUp {
    type Response = ();

    fn path(&self) -> String {
        "/api/auth/signup".into()
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Enroll(pub String);

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Chat {
    /// The user who sends the message.
    pub sender: String,
    /// The chat message.
    pub msg: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ListPlayer;

impl Req for ListPlayer {
    type Response = Vec<String>;

    fn path(&self) -> String {
        "/api/player".into()
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GetPlayer(#[serde(skip)] pub String);

impl Req for GetPlayer {
    type Response = User;

    fn path(&self) -> String {
        format!("/api/player/{}", self.0)
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct EditPlayer(#[serde(skip)] pub String, pub User);

impl Req for EditPlayer {
    type Response = ();

    fn path(&self) -> String {
        format!("/api/player/{}", self.0)
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ListPC;

impl Req for ListPC {
    type Response = Vec<(String, PC)>;

    fn path(&self) -> String {
        "/api/pc".into()
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GetPC(#[serde(skip)] pub String);

impl Req for GetPC {
    type Response = PC;

    fn path(&self) -> String {
        format!("/api/pc/{}", self.0)
    }

    const METHOD: &'static str = "GET";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct EditPC(#[serde(skip)] pub String, pub Option<PC>);

impl Req for EditPC {
    type Response = ();

    fn path(&self) -> String {
        format!("/api/pc/{}", self.0)
    }

    const METHOD: &'static str = "POST";
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct EditPass(pub String);

impl Req for EditPass {
    type Response = ();

    fn path(&self) -> String {
        "/api/auth/pass".into()
    }

    const METHOD: &'static str = "POST";
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Sync {}
