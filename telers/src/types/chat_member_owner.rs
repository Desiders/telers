use super::User;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that owns the chat and has all administrator privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMemberOwner {
    /// Information about the user
    pub user: User,
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<Box<str>>,
}
