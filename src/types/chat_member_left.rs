use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>`_ that isn't currently a member of the chat, but may join it themselves.
/// <https://core.telegram.org/bots/api#chatmemberleft>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always 'left'
    #[serde(default = "left")]
    pub status: String,
    /// Information about the user
    pub user: User,
}

impl Default for ChatMemberLeft {
    fn default() -> Self {
        Self {
            status: left(),
            user: User::default(),
        }
    }
}

fn left() -> String {
    "left".to_string()
}
