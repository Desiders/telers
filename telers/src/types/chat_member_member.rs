use super::User;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that has no additional privileges or restrictions.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMemberMember {
    /// Information about the user
    pub user: User,
}
