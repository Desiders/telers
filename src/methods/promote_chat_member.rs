use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass `False` for all boolean parameters to demote a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#promotechatmember>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target group or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass `True` if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// Pass `True`, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: Option<bool>,
    /// Pass `True`, if the administrator can create channel posts, channels only
    pub can_post_messages: Option<bool>,
    /// Pass `True`, if the administrator can edit messages of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Pass `True`, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Pass `True`, if the administrator can manage video chats
    pub can_manage_voice_chats: Option<bool>,
    /// Pass `True`, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Pass `True`, if the administrator can add new administrators with a subset of his own privileges or demote administrators that they has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>,
    /// Pass `True`, if the administrator can change the chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Pass `True`, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Pass `True` if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    /// Pass `True` if the user is allowed to create, rename, close, and reopen forum topics, supergroups only
    pub can_manage_topics: Option<bool>,
}

impl PromoteChatMember {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_voice_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn is_anonymous(mut self, val: bool) -> Self {
        self.is_anonymous = Some(val);
        self
    }

    #[must_use]
    pub fn can_manage_chat(mut self, val: bool) -> Self {
        self.can_manage_chat = Some(val);
        self
    }

    #[must_use]
    pub fn can_post_messages(mut self, val: bool) -> Self {
        self.can_post_messages = Some(val);
        self
    }

    #[must_use]
    pub fn can_edit_messages(mut self, val: bool) -> Self {
        self.can_edit_messages = Some(val);
        self
    }

    #[must_use]
    pub fn can_delete_messages(mut self, val: bool) -> Self {
        self.can_delete_messages = Some(val);
        self
    }

    #[must_use]
    pub fn can_manage_voice_chats(mut self, val: bool) -> Self {
        self.can_manage_voice_chats = Some(val);
        self
    }

    #[must_use]
    pub fn can_restrict_members(mut self, val: bool) -> Self {
        self.can_restrict_members = Some(val);
        self
    }

    #[must_use]
    pub fn can_promote_members(mut self, val: bool) -> Self {
        self.can_promote_members = Some(val);
        self
    }

    #[must_use]
    pub fn can_change_info(mut self, val: bool) -> Self {
        self.can_change_info = Some(val);
        self
    }

    #[must_use]
    pub fn can_invite_users(mut self, val: bool) -> Self {
        self.can_invite_users = Some(val);
        self
    }

    #[must_use]
    pub fn can_pin_messages(mut self, val: bool) -> Self {
        self.can_pin_messages = Some(val);
        self
    }

    #[must_use]
    pub fn can_manage_topics(mut self, val: bool) -> Self {
        self.can_manage_topics = Some(val);
        self
    }
}

impl TelegramMethod for PromoteChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("promoteChatMember", self, None)
    }
}
