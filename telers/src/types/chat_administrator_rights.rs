use serde::{Deserialize, Serialize};
/// Represents the rights of an administrator in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatadministratorrights>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// `true`, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    pub can_manage_chat: bool,
    /// `true`, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// `true`, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// `true`, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: bool,
    /// `true`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// `true`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// `true`, if the administrator can post stories to the chat
    pub can_post_stories: bool,
    /// `true`, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: bool,
    /// `true`, if the administrator can delete stories posted by other users
    pub can_delete_stories: bool,
    /// `true`, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// `true`, if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// `true`, if the user is allowed to pin messages; for groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// `true`, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// `true`, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// `true`, if the administrator can edit the tags of regular members; for groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
    /// `true`, if the administrator can manage chat welcome messages or directly send them in the case of bots
    pub can_send_welcome_messages: bool,
}
impl ChatAdministratorRights {
    /// Creates a new `ChatAdministratorRights`.
    ///
    /// # Arguments
    /// * `is_anonymous` - `true`, if the user's presence in the chat is hidden
    /// * `can_manage_chat` - `true`, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    /// * `can_delete_messages` - `true`, if the administrator can delete messages of other users
    /// * `can_manage_video_chats` - `true`, if the administrator can manage video chats
    /// * `can_restrict_members` - `true`, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    /// * `can_promote_members` - `true`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    /// * `can_change_info` - `true`, if the user is allowed to change the chat title, photo and other settings
    /// * `can_invite_users` - `true`, if the user is allowed to invite new users to the chat
    /// * `can_post_stories` - `true`, if the administrator can post stories to the chat
    /// * `can_edit_stories` - `true`, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    /// * `can_delete_stories` - `true`, if the administrator can delete stories posted by other users
    /// * `can_send_welcome_messages` - `true`, if the administrator can manage chat welcome messages or directly send them in the case of bots
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<bool>,
        T1: Into<bool>,
        T2: Into<bool>,
        T3: Into<bool>,
        T4: Into<bool>,
        T5: Into<bool>,
        T6: Into<bool>,
        T7: Into<bool>,
        T8: Into<bool>,
        T9: Into<bool>,
        T10: Into<bool>,
        T11: Into<bool>,
    >(
        is_anonymous: T0,
        can_manage_chat: T1,
        can_delete_messages: T2,
        can_manage_video_chats: T3,
        can_restrict_members: T4,
        can_promote_members: T5,
        can_change_info: T6,
        can_invite_users: T7,
        can_post_stories: T8,
        can_edit_stories: T9,
        can_delete_stories: T10,
        can_send_welcome_messages: T11,
    ) -> Self {
        Self {
            is_anonymous: is_anonymous.into(),
            can_manage_chat: can_manage_chat.into(),
            can_delete_messages: can_delete_messages.into(),
            can_manage_video_chats: can_manage_video_chats.into(),
            can_restrict_members: can_restrict_members.into(),
            can_promote_members: can_promote_members.into(),
            can_change_info: can_change_info.into(),
            can_invite_users: can_invite_users.into(),
            can_post_stories: can_post_stories.into(),
            can_edit_stories: can_edit_stories.into(),
            can_delete_stories: can_delete_stories.into(),
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_manage_topics: None,
            can_manage_direct_messages: None,
            can_manage_tags: None,
            can_send_welcome_messages: can_send_welcome_messages.into(),
        }
    }

    /// `true`, if the user's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_anonymous = val.into();
        self
    }

    /// `true`, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[must_use]
    pub fn can_manage_chat<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_chat = val.into();
        self
    }

    /// `true`, if the administrator can delete messages of other users
    #[must_use]
    pub fn can_delete_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_messages = val.into();
        self
    }

    /// `true`, if the administrator can manage video chats
    #[must_use]
    pub fn can_manage_video_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_video_chats = val.into();
        self
    }

    /// `true`, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    #[must_use]
    pub fn can_restrict_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_restrict_members = val.into();
        self
    }

    /// `true`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    #[must_use]
    pub fn can_promote_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_promote_members = val.into();
        self
    }

    /// `true`, if the user is allowed to change the chat title, photo and other settings
    #[must_use]
    pub fn can_change_info<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_change_info = val.into();
        self
    }

    /// `true`, if the user is allowed to invite new users to the chat
    #[must_use]
    pub fn can_invite_users<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_invite_users = val.into();
        self
    }

    /// `true`, if the administrator can post stories to the chat
    #[must_use]
    pub fn can_post_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_post_stories = val.into();
        self
    }

    /// `true`, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[must_use]
    pub fn can_edit_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_stories = val.into();
        self
    }

    /// `true`, if the administrator can delete stories posted by other users
    #[must_use]
    pub fn can_delete_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_stories = val.into();
        self
    }

    /// `true`, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_post_messages = Some(val.into());
        self
    }

    /// `true`, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_post_messages = val.map(Into::into);
        self
    }

    /// `true`, if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_messages = Some(val.into());
        self
    }

    /// `true`, if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_messages = val.map(Into::into);
        self
    }

    /// `true`, if the user is allowed to pin messages; for groups and supergroups only
    #[must_use]
    pub fn can_pin_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_pin_messages = Some(val.into());
        self
    }

    /// `true`, if the user is allowed to pin messages; for groups and supergroups only
    #[must_use]
    pub fn can_pin_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_pin_messages = val.map(Into::into);
        self
    }

    /// `true`, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[must_use]
    pub fn can_manage_topics<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_topics = Some(val.into());
        self
    }

    /// `true`, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[must_use]
    pub fn can_manage_topics_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_topics = val.map(Into::into);
        self
    }

    /// `true`, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[must_use]
    pub fn can_manage_direct_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_direct_messages = Some(val.into());
        self
    }

    /// `true`, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[must_use]
    pub fn can_manage_direct_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_direct_messages = val.map(Into::into);
        self
    }

    /// `true`, if the administrator can edit the tags of regular members; for groups and supergroups only
    #[must_use]
    pub fn can_manage_tags<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_tags = Some(val.into());
        self
    }

    /// `true`, if the administrator can edit the tags of regular members; for groups and supergroups only
    #[must_use]
    pub fn can_manage_tags_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_tags = val.map(Into::into);
        self
    }

    /// `true`, if the administrator can manage chat welcome messages or directly send them in the case of bots
    #[must_use]
    pub fn can_send_welcome_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_send_welcome_messages = val.into();
        self
    }
}
