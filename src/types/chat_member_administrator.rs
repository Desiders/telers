use super::User;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that has some additional privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberadministrator>
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,
    /// `true`, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// `true`, if the administrator can access the chat event log, chat statistics, boost list in channels, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// `true`, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// `true`, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// `true`, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// `true`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// `true`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// `true`, if the administrator can post messages in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// `true`, if the administrator can edit messages of other users and can pin messages; channels only
    pub can_edit_messages: Option<bool>,
    /// `true`, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// `true`, if the administrator can post stories in the channel; channels only
    pub can_post_stories: Option<bool>,
    /// `true`, if the administrator can edit stories posted by other users; channels only
    pub can_edit_stories: Option<bool>,
    /// `true`, if the administrator can delete stories posted by other users; channels only
    pub can_delete_stories: Option<bool>,
    /// `true`, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    pub can_manage_topics: Option<bool>,
    /// Custom title for this user
    pub custom_title: Option<Box<str>>,
}
