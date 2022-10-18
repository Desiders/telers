use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>`_ that owns the chat and has all administrator privileges.
/// <https://core.telegram.org/bots/api#chatmemberowner>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always 'creator'
    #[serde(default = "creator")]
    pub status: String,
    /// Information about the user
    pub user: User,
    /// :code:`True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// *Optional*. Custom title for this user
    pub custom_title: Option<String>,
}

fn creator() -> String {
    "creator".to_string()
}
