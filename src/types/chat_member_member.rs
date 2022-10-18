use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>`_ that has no additional privileges or restrictions.
/// <https://core.telegram.org/bots/api#chatmembermember>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always 'member'
    #[serde(default = "member")]
    pub status: String,
    /// Information about the user
    pub user: User,
}

fn member() -> String {
    "member".to_string()
}
