use crate::client::Bot;
use serde::Serialize;
/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass `false` for all boolean parameters to demote a user. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#promotechatmember>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass `true` if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass `true` if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass `true` if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass `true` if the administrator can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Pass `true` if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to `true` for promotions of channel administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass `true` if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass `true` if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass `true` if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass `true` if the administrator can post stories to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// Pass `true` if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// Pass `true` if the administrator can delete stories posted by other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// Pass `true` if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass `true` if the administrator can edit messages of other users and can pin messages; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass `true` if the administrator can pin messages; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Pass `true` if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Pass `true` if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// Pass `true` if the administrator can edit the tags of regular members; for groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
}
impl PromoteChatMember {
    /// Creates a new `PromoteChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        user_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
            is_anonymous: None,
            can_manage_chat: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_manage_topics: None,
            can_manage_direct_messages: None,
            can_manage_tags: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Pass `true` if the administrator's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_anonymous = Some(val.into());
        self
    }

    /// Pass `true` if the administrator's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_anonymous = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[must_use]
    pub fn can_manage_chat<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_chat = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[must_use]
    pub fn can_manage_chat_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_chat = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can delete messages of other users
    #[must_use]
    pub fn can_delete_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_messages = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can delete messages of other users
    #[must_use]
    pub fn can_delete_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_delete_messages = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can manage video chats
    #[must_use]
    pub fn can_manage_video_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_video_chats = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can manage video chats
    #[must_use]
    pub fn can_manage_video_chats_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_video_chats = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to `true` for promotions of channel administrators.
    #[must_use]
    pub fn can_restrict_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_restrict_members = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can restrict, ban or unban chat members, or access supergroup statistics. For backward compatibility, defaults to `true` for promotions of channel administrators.
    #[must_use]
    pub fn can_restrict_members_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_restrict_members = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[must_use]
    pub fn can_promote_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_promote_members = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[must_use]
    pub fn can_promote_members_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_promote_members = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can change chat title, photo and other settings
    #[must_use]
    pub fn can_change_info<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_change_info = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can change chat title, photo and other settings
    #[must_use]
    pub fn can_change_info_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_change_info = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can invite new users to the chat
    #[must_use]
    pub fn can_invite_users<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_invite_users = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can invite new users to the chat
    #[must_use]
    pub fn can_invite_users_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_invite_users = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can post stories to the chat
    #[must_use]
    pub fn can_post_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_post_stories = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can post stories to the chat
    #[must_use]
    pub fn can_post_stories_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_post_stories = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[must_use]
    pub fn can_edit_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_stories = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[must_use]
    pub fn can_edit_stories_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_stories = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can delete stories posted by other users
    #[must_use]
    pub fn can_delete_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_stories = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can delete stories posted by other users
    #[must_use]
    pub fn can_delete_stories_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_delete_stories = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_post_messages = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_post_messages = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_messages = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_messages = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can pin messages; for supergroups only
    #[must_use]
    pub fn can_pin_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_pin_messages = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can pin messages; for supergroups only
    #[must_use]
    pub fn can_pin_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_pin_messages = val.map(Into::into);
        self
    }

    /// Pass `true` if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[must_use]
    pub fn can_manage_topics<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_topics = Some(val.into());
        self
    }

    /// Pass `true` if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    #[must_use]
    pub fn can_manage_topics_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_topics = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    #[must_use]
    pub fn can_manage_direct_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_direct_messages = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can manage direct messages within the channel and decline suggested posts; for channels only
    #[must_use]
    pub fn can_manage_direct_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_direct_messages = val.map(Into::into);
        self
    }

    /// Pass `true` if the administrator can edit the tags of regular members; for groups and supergroups only
    #[must_use]
    pub fn can_manage_tags<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_tags = Some(val.into());
        self
    }

    /// Pass `true` if the administrator can edit the tags of regular members; for groups and supergroups only
    #[must_use]
    pub fn can_manage_tags_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_tags = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for PromoteChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("promoteChatMember", self, None)
    }
}
