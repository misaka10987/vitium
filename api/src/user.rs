use serde::{Deserialize, Serialize};

/// A user profile.
///
/// This includes information associated with a certain user.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Email address.
    ///
    /// This shall be used for account options like password resetting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Optional self introduction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
}
