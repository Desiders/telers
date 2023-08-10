use super::{Chat, ChatInviteLink, ChatMember, Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object represents changes in the status of a chat member.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberupdated>
/// # Warnings
/// This structure has so big size, so it's recommended to use it inside [`std::sync::Arc`], [`Box`] and other smart pointers
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
    /// `True`, if the user joined the chat via a chat folder invite link
    pub via_chat_folder_invite_link: Option<bool>,
}

impl ChatMemberUpdated {
    /// Gets the chat ID from the chat member updated
    #[must_use]
    pub const fn chat_id(&self) -> i64 {
        self.chat.id
    }

    /// Gets the user ID from the chat member updated
    #[must_use]
    pub const fn user_id(&self) -> i64 {
        self.from.id
    }
}

impl TryFrom<Update> for ChatMemberUpdated {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(chat_member_updated) = update.my_chat_member {
            Ok(chat_member_updated)
        } else if let Some(chat_member_updated) = update.chat_member {
            Ok(chat_member_updated)
        } else {
            Err(ConvertUpdateToTypeError::new("ChatMemberUpdated"))
        }
    }
}
