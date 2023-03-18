use super::User;

use crate::enums::ChatMemberStatus;

use serde::Deserialize;

/// Represents a [`chat member`](https://core.telegram.org/bots/api#chatmember) that has no additional privileges or restrictions.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberMember {
    /// The member's status in the chat, always 'member'
    #[serde(default = "member")]
    pub status: String,
    /// Information about the user
    pub user: User,
}

fn member() -> String {
    ChatMemberStatus::Member.into()
}
