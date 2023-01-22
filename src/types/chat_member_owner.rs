use super::User;

use serde::Deserialize;

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>` that owns the chat and has all administrator privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always 'creator'
    pub status: String,
    /// Information about the user
    pub user: User,
    /// `True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// *Optional*. Custom title for this user
    pub custom_title: Option<String>,
}
