use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct CommandLine {
    pub user: Option<String>,
    pub line: String,
}

#[cfg_attr(target_family = "wasm", tsify_next::declare)]
pub type CommandStatus = Result<String, String>;
