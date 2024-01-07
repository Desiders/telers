use super::{Chat, ReactionCount, Update, UpdateKind};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object represents reaction changes on a message with anonymous reactions.
/// # Documentation
/// <https://core.telegram.org/bots/api#messagereactioncountupdated>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Chat,
    /// Unique message identifier inside the chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Date of the change in Unix time
    pub date: i64,
    /// List of reactions that are present on the message
    pub reactions: Box<[ReactionCount]>,
}

impl TryFrom<Update> for MessageReactionCountUpdated {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::MessageReactionCount(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "MessageReactionCount")),
        }
    }
}
