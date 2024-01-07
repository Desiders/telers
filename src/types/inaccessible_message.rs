use super::Chat;

use serde::Deserialize;

/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#inaccessiblemessage>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Chat,
    /// Unique message identifier inside the chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,
}
