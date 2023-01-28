use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatadministratorcustomtitle>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target group or username of the target supergroup (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, T: Into<String>>(
        chat_id: C,
        user_id: i64,
        custom_title: T,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
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
    pub fn custom_title<T: Into<String>>(mut self, val: T) -> Self {
        self.custom_title = val.into();
        self
    }
}

impl TelegramMethod for SetChatAdministratorCustomTitle {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("SetChatAdministratorCustomTitle", self, None)
    }
}
