use super::{Chat, ChatInviteLink, Update, User};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a join request sent to a chat.
/// <https://core.telegram.org/bots/api#chatjoinrequest>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl From<Update> for ChatJoinRequest {
    fn from(update: Update) -> Self {
        update
            .chat_join_request
            .expect("Update isn't a `ChatJoinRequest`")
    }
}
