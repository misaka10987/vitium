use serde::{Deserialize, Serialize};

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UserProfile {
    /// The displayed name in contrast to username used for login.
    pub nickname: String,
    /// URL to the avatar image.
    pub avatar: Option<String>,
    /// Optional email.
    pub email: Option<String>,
    /// Optional self introduction.
    pub intro: Option<String>,
}
