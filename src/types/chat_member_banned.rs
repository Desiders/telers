use super::User;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that was banned in the chat and can't return to the chat or view chat messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberbanned>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMemberBanned {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}
