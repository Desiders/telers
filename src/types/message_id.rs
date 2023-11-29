use serde::Deserialize;

/// This object represents a unique message identifier.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageid>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    #[serde(rename = "message_id")]
    pub id: i64,
}

impl From<MessageId> for i64 {
    fn from(val: MessageId) -> Self {
        val.id
    }
}
