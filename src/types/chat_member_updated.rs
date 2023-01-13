use super::{Chat, ChatInviteLink, ChatMember, ChatMemberMember, Update, User};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents changes in the status of a chat member.
/// <https://core.telegram.org/bots/api#chatmemberupdated>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    /// *Optional*. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}

impl Default for ChatMemberUpdated {
    fn default() -> Self {
        Self {
            chat: Chat::default(),
            from: User::default(),
            date: 0,
            old_chat_member: ChatMember::Member(ChatMemberMember::default()),
            new_chat_member: ChatMember::Member(ChatMemberMember::default()),
            invite_link: None,
        }
    }
}

impl From<Update> for ChatMemberUpdated {
    fn from(update: Update) -> Self {
        update
            .my_chat_member
            .or(update.chat_member)
            .expect("Update doesn't contain a `ChatMemberUpdated`")
    }
}
