use serde::{Deserialize, Serialize};

/// HTML form definition for a signup request.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct SignUp {
    /// The username to create.
    pub user: String,
    /// The initial password.
    pub pass: String,
}
