use super::{Chat, ChatInviteLink, Update, User};

use crate::errors::ConvertUpdateToTypeError;

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
    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 24 hours to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

impl ChatJoinRequest {
    /// Gets the chat ID from the chat join request
    #[must_use]
    pub const fn chat_id(&self) -> i64 {
        self.chat.id
    }

    /// Gets the sender user ID from the chat join request
    #[must_use]
    pub const fn sender_user_id(&self) -> i64 {
        self.from.id
    }

    /// Gets the sender user ID from the chat join request
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> i64 {
        self.sender_user_id()
    }
}

impl TryFrom<Update> for ChatJoinRequest {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(chat_join_request) = update.chat_join_request {
            Ok(chat_join_request)
        } else {
            Err(ConvertUpdateToTypeError::new("ChatJoinRequest"))
        }
    }
}
