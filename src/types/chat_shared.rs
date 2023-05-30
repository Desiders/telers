use serde::Deserialize;

/// This object contains information about the chat whose identifier was shared with the bot using a [`KeyboardButtonRequestChat`](crate::types::KeyboardButtonRequestChat) button.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatshared>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,
}
