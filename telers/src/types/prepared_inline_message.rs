use serde::{Deserialize, Serialize};

/// Describes an inline message to be sent by a user of a Mini App.
/// # Documentation
/// <https://core.telegram.org/bots/api#preparedinlinemessage>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: Box<str>,
    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used
    pub expiration_date: i64,
}
