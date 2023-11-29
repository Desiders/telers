use super::{Chat, ChatInviteLink, Update, UpdateKind, User};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// Represents a join request sent to a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatjoinrequest>
/// # Warnings
/// This structure has so big size, so it's recommended to use it inside [`std::sync::Arc`], [`Box`] and other smart pointers
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
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
    pub bio: Option<Box<str>>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}

impl TryFrom<Update> for ChatJoinRequest {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::ChatJoinRequest(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ChatJoinRequest")),
        }
    }
}
