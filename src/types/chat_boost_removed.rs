use super::{Chat, ChatBoostSource, Update, UpdateKind};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object represents a boost removed from a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostremoved>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Chat,
    /// Unique identifier of the boost
    #[serde(rename = "boost_id")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the boost was removed
    pub remove_date: i32,
    /// Source of the removed boost
    pub source: ChatBoostSource,
}

impl TryFrom<Update> for ChatBoostRemoved {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::RemovedChatBoost(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ChatBoostRemoved")),
        }
    }
}
