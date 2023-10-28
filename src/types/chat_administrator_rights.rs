use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the rights of an administrator in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatadministratorrights>
#[allow(clippy::struct_excessive_bools)]
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatAdministratorRights {
    /// `True`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// `True`, if the administrator can access the chat event log, chat statistics, boost list in channels, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
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
    /// `True`, if the administrator can post messages in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// `True`, if the administrator can edit messages of other users and can pin messages; channels only
    pub can_edit_messages: Option<bool>,
    /// `True`, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// `True`, if the administrator can post stories in the channel; channels only
    pub can_post_stories: Option<bool>,
    /// `True`, if the administrator can edit stories posted by other users; channels only
    pub can_edit_stories: Option<bool>,
    /// `True`, if the administrator can delete stories posted by other users; channels only
    pub can_delete_stories: Option<bool>,
    /// `True`, if the user is allowed to create, rename, close, and reopen forum topics; supergroups only
    pub can_manage_topics: Option<bool>,
}

impl ChatAdministratorRights {
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    pub fn new(
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
    ) -> Self {
        Self {
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_manage_topics: None,
        }
    }

    #[must_use]
    pub fn is_anonymous(self, val: bool) -> Self {
        Self {
            is_anonymous: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_chat(self, val: bool) -> Self {
        Self {
            can_manage_chat: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_delete_messages(self, val: bool) -> Self {
        Self {
            can_delete_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_video_chats(self, val: bool) -> Self {
        Self {
            can_manage_video_chats: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_restrict_members(self, val: bool) -> Self {
        Self {
            can_restrict_members: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_promote_members(self, val: bool) -> Self {
        Self {
            can_promote_members: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info(self, val: bool) -> Self {
        Self {
            can_change_info: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users(self, val: bool) -> Self {
        Self {
            can_invite_users: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_post_messages(self, val: bool) -> Self {
        Self {
            can_post_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_edit_messages(self, val: bool) -> Self {
        Self {
            can_edit_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_pin_messages(self, val: bool) -> Self {
        Self {
            can_pin_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_post_stories(self, val: bool) -> Self {
        Self {
            can_post_stories: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_edit_stories(self, val: bool) -> Self {
        Self {
            can_edit_stories: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_delete_stories(self, val: bool) -> Self {
        Self {
            can_delete_stories: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_topics(self, val: bool) -> Self {
        Self {
            can_manage_topics: Some(val),
            ..self
        }
    }
}
