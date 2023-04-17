use super::{Chat, ChatInviteLink, Update, User};

use crate::error::ConvertUpdateToTypeError;

use serde::Deserialize;

/// Represents a join request sent to a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatjoinrequest>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

impl TryFrom<Update> for ChatJoinRequest {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(chat_join_request) = update.chat_join_request {
            Ok(chat_join_request)
        } else {
            Err(ConvertUpdateToTypeError::new(format!(
                "Update `{update:?}` doesn't contain `ChatJoinRequest`"
            )))
        }
    }
}
