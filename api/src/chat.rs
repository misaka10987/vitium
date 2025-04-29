use serde::{Deserialize, Serialize};

/// A global out-game chat message.
///
/// # Time Order
///
/// The [`Self::sender`] fields indicates the local timestamp on the client when and where this message is sent.
/// No guarantee is made about the consistency of the time order of message.
///
/// # HTML
///
/// The message can either be a plain text message (when [`Self::html`] is set to `false`)
/// or an HTML message (when [`Self::html`] is set to `true`).
///
/// ## Caution
///
/// Only constrait to [`Self::content`] is the same as that of [`String`], i.e. to be valid UTF-8,
/// even if [`Self::html`] is on.
/// Therefore, no guarantee is made about whether it is valid DOM element or HTML.
/// Appropriate checks shall be done before attempting to render HTML to avoid potential danger.
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
pub struct Message {
    /// Milisecond UNIX timestamp when the message is sent.
    #[cfg(target_family = "wasm")] // walkaround for json missing bigint support
    pub time: f64,
    /// Milisecond UNIX timestamp when the message is sent.
    #[cfg(not(target_family = "wasm"))]
    pub time: u64,
    /// The user who sends the message.
    ///
    /// A [`None`] indicates that this message is a broadcast triggered by a server command.
    pub sender: Option<String>,
    /// Content of the chat.
    /// This field shall be interpreted with respect to [`Self::html`] as plain text or string of HTML.
    pub content: String,
    /// Whether HTML is enabled in the content of the message.
    pub html: bool,
}
