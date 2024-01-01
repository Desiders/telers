use super::{Chat, ChatBoostSource};

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
