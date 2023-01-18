use super::User;

use serde::Deserialize;

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>` that isn't currently a member of the chat, but may join it themselves.
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always 'left'
    pub status: String,
    /// Information about the user
    pub user: User,
}
