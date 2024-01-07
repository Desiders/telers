use super::User;

use serde::Deserialize;

/// The message was originally sent by a known user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginuser>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct MessageOriginUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// User that sent the message originally
    pub sender_user: User,
}
