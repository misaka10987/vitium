pub use crate::game::Action;
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

/// Denotes a payload that is accessed with a REST API.
pub trait REST: Serialize + DeserializeOwned {
    /// The index type that identifies individual resource from collection, e.g. username for users.
    type Index;
    /// The path on the server to send request to.
    fn path() -> String;
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
pub struct Message {
    /// Milisecond timestamp the message is sent.
    pub time: u64,
    /// The user who sends the message.
    pub sender: String,
    /// The chat message.
    pub content: String,
    /// Where to enable HTML support in message body.
    pub html: bool,
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
