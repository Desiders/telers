use super::User;

use serde::Deserialize;

/// Represents a [`chat member`](https://core.telegram.org/bots/api#chatmember) that was banned in the chat and can't return to the chat or view chat messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberbanned>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always 'kicked'
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is banned forever
    pub until_date: i64,
}
