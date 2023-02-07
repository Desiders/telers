use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter `only_if_banned`.
/// # Documentation
/// <https://core.telegram.org/bots/api#unbanchatmember>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    pub only_if_banned: Option<bool>,
}

impl UnbanChatMember {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
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
    pub fn only_if_banned(mut self, val: bool) -> Self {
        self.only_if_banned = Some(val);
        self
    }
}

impl TelegramMethod for UnbanChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("unbanChatMember", self, None)
    }
}
