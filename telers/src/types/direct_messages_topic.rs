use crate::types::User;

use serde::{Deserialize, Serialize};

/// Describes a topic of a direct messages chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#directmessagestopic>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub topic_id: i64,
    /// Information about the user that created the topic. Currently, it is always present
    pub user: Option<User>,
}
