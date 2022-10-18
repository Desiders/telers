use super::{Chat, ChatInviteLink, User};

use serde::{Deserialize, Serialize};

/// Represents a join request sent to a chat.
/// <https://core.telegram.org/bots/api#chatjoinrequest>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// *Optional*. Bio of the user.
    pub bio: Option<String>,
    /// *Optional*. Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}
