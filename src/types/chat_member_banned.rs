use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>`_ that was banned in the chat and can't return to the chat or view chat messages.
/// <https://core.telegram.org/bots/api#chatmemberbanned>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always 'kicked'
    #[serde(default = "kicked")]
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is banned forever
    pub until_date: i64,
}

impl Default for ChatMemberBanned {
    fn default() -> Self {
        Self {
            status: kicked(),
            user: User::default(),
            until_date: 0,
        }
    }
}

fn kicked() -> String {
    "kicked".to_string()
}
