use serde::{Deserialize, Serialize};

/// A line of command to be sent and executed by the server.
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(
    target_family = "wasm",
    tsify(
        into_wasm_abi,
        from_wasm_abi,
        missing_as_null,
        large_number_types_as_bigints
    )
)]
pub struct CommandLine {
    /// The user who issued this command.
    ///
    /// A value of [`None`] indicates that the command is directly executed from the server console.
    pub user: Option<String>,
    /// The complete line of command to execute, including the name of command and space seperated arguments.
    pub line: String,
}

/// Denotes the exit status of a command.
///
/// An [`Ok`] indicates that the command exits with no error, with a payload of the command output.
/// An [`Err`] indicates that the command exits with error, or is not executed
/// (e.g. the command issuer does not have sufficient permission), with a payload of the formatted error message.
///
/// # Payload
/// The payload would be a string that might contain [ANSI Escape Color Codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
/// Therefore, it is recommended to perform corresponding checks before printing the message.
#[cfg_attr(target_family = "wasm", tsify_next::declare)]
pub type CommandStatus = Result<String, String>;
