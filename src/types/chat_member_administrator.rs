use super::User;

use crate::enums::ChatMemberStatus;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that has some additional privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberadministrator>
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always 'administrator'
    #[serde(default = "administrator")]
    pub status: String,
    /// Information about the user
    pub user: User,
    /// `True`, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// `True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// `True`, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// `True`, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// `True`, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// `True`, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// `True`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// `True`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// `True`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// `True`, if the administrator can post in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// `True`, if the administrator can edit messages of other users and can pin messages; channels only
    pub can_edit_messages: Option<bool>,
    /// `True`, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

fn administrator() -> String {
    ChatMemberStatus::Administrator.into()
}
