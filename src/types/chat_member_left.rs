use super::User;

use crate::enums::ChatMemberStatus;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that isn't currently a member of the chat, but may join it themselves.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always 'left'
    #[serde(default = "left")]
    pub status: Box<str>,
    /// Information about the user
    pub user: User,
}

fn left() -> Box<str> {
    ChatMemberStatus::Left.into()
}
