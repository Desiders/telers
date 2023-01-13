use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>` that owns the chat and has all administrator privileges.
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// The member's status in the chat, always 'creator'
    #[serde(default = "creator")]
    pub status: String,
    /// Information about the user
    pub user: User,
    /// `True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// *Optional*. Custom title for this user
    pub custom_title: Option<String>,
}

impl Default for ChatMemberOwner {
    fn default() -> Self {
        Self {
            status: creator(),
            user: User::default(),
            is_anonymous: false,
            custom_title: None,
        }
    }
}

fn creator() -> String {
    "creator".to_string()
}
