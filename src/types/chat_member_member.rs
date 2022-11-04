use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>` that has no additional privileges or restrictions.
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always 'member'
    #[serde(default = "member")]
    pub status: String,
    /// Information about the user
    pub user: User,
}

impl Default for ChatMemberMember {
    fn default() -> Self {
        Self {
            status: member(),
            user: User::default(),
        }
    }
}

fn member() -> String {
    "member".to_string()
}
