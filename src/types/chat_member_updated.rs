use super::{Chat, ChatInviteLink, ChatMember, User};

use serde::{Deserialize, Serialize};

/// This object represents changes in the status of a chat member.
/// <https://core.telegram.org/bots/api#chatmemberupdated>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    chat: Chat,
    /// Performer of the action, which resulted in the change
    from: User,
    /// Date the change was done in Unix time
    date: i64,
    /// Previous information about the chat member
    old_chat_member: ChatMember,
    /// New information about the chat member
    new_chat_member: ChatMember,
    /// *Optional*. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    invite_link: Option<ChatInviteLink>,
}
