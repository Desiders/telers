use serde::Deserialize;

/// This object represents a unique message identifier. \
/// <https://core.telegram.org/bots/api#messageid>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i64,
}

impl From<MessageId> for i64 {
    fn from(message_id: MessageId) -> Self {
        message_id.message_id
    }
}
