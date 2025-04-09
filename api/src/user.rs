use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
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
