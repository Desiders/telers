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
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target group or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass `True` if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// Pass `True`, if the administrator can access the chat event log, chat statistics, boost list in channels, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
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
    /// Pass `True`, if the administrator can post stories in the channel; channels only
    pub can_post_stories: Option<bool>,
    /// Pass `True`, if the administrator can edit stories posted by other users; channels only
    pub can_edit_stories: Option<bool>,
    /// Pass `True`` if the administrator can delete stories posted by other users; channels only
    pub can_delete_stories: Option<bool>,
    /// Pass `True` if the user is allowed to create, rename, close, and reopen forum topics, supergroups only
    pub can_manage_topics: Option<bool>,
}

impl PromoteChatMember {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64) -> Self {
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
            can_post_stories: None,
            can_edit_stories: None,
            can_delete_stories: None,
            can_manage_topics: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn is_anonymous(self, val: bool) -> Self {
        Self {
            is_anonymous: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_chat(self, val: bool) -> Self {
        Self {
            can_manage_chat: Some(val),
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
    pub fn can_delete_messages(self, val: bool) -> Self {
        Self {
            can_delete_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_voice_chats(self, val: bool) -> Self {
        Self {
            can_manage_voice_chats: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_restrict_members(self, val: bool) -> Self {
        Self {
            can_restrict_members: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_promote_members(self, val: bool) -> Self {
        Self {
            can_promote_members: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info(self, val: bool) -> Self {
        Self {
            can_change_info: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users(self, val: bool) -> Self {
        Self {
            can_invite_users: Some(val),
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
    pub fn can_manage_topics(self, val: bool) -> Self {
        Self {
            can_manage_topics: Some(val),
            ..self
        }
    }
}

impl PromoteChatMember {
    #[must_use]
    pub fn is_anonymous_option(self, val: Option<bool>) -> Self {
        Self {
            is_anonymous: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_chat_option(self, val: Option<bool>) -> Self {
        Self {
            can_manage_chat: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_post_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_post_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_edit_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_edit_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_delete_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_delete_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_voice_chats_option(self, val: Option<bool>) -> Self {
        Self {
            can_manage_voice_chats: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_restrict_members_option(self, val: Option<bool>) -> Self {
        Self {
            can_restrict_members: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_promote_members_option(self, val: Option<bool>) -> Self {
        Self {
            can_promote_members: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info_option(self, val: Option<bool>) -> Self {
        Self {
            can_change_info: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users_option(self, val: Option<bool>) -> Self {
        Self {
            can_invite_users: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_pin_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_pin_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_topics_option(self, val: Option<bool>) -> Self {
        Self {
            can_manage_topics: val,
            ..self
        }
    }
}

impl TelegramMethod for PromoteChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("promoteChatMember", self, None)
    }
}

impl AsRef<PromoteChatMember> for PromoteChatMember {
    fn as_ref(&self) -> &Self {
        self
    }
}
