use super::ChatAdministratorRights;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object defines the criteria used to request a suitable chat. The identifier of the selected chat will be shared with the bot when the corresponding button is pressed. [`More about requesting chats`](https://core.telegram.org/bots/features#chat-and-user-selection)
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestchat>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the [`ChatShared`](crate::types::ChatShared) object. Must be unique within the message
    pub request_id: i32,
    /// Pass `true` to request a channel chat, pass `false` to request a group or a supergroup chat.
    pub chat_is_channel: bool,
    /// Pass `true` to request a forum supergroup, pass `false` to request a non-forum chat. If not specified, no additional restrictions are applied.
    pub chat_is_forum: Option<bool>,
    /// Pass `true` to request a supergroup or a channel with a username, pass `false` to request a chat without a username. If not specified, no additional restrictions are applied.
    pub chat_has_username: Option<bool>,
    /// Pass `true` to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    pub chat_is_created: Option<bool>,
    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of `bot_administrator_rights`. If not specified, no additional restrictions are applied.
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of `user_administrator_rights`. If not specified, no additional restrictions are applied.
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass `true` to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    pub bot_is_member: Option<bool>,
}

impl KeyboardButtonRequestChat {
    #[must_use]
    pub fn new(request_id: i32, chat_is_channel: bool) -> Self {
        Self {
            request_id,
            chat_is_channel,
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: None,
        }
    }

    #[must_use]
    pub fn request_id(self, val: i32) -> Self {
        Self {
            request_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_is_channel(self, val: bool) -> Self {
        Self {
            chat_is_channel: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_is_forum(self, val: bool) -> Self {
        Self {
            chat_is_forum: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn chat_has_username(self, val: bool) -> Self {
        Self {
            chat_has_username: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn chat_is_created(self, val: bool) -> Self {
        Self {
            chat_is_created: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn user_administrator_rights(self, val: impl Into<ChatAdministratorRights>) -> Self {
        Self {
            user_administrator_rights: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn bot_administrator_rights(self, val: impl Into<ChatAdministratorRights>) -> Self {
        Self {
            bot_administrator_rights: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn bot_is_member(self, val: bool) -> Self {
        Self {
            bot_is_member: Some(val),
            ..self
        }
    }
}

impl KeyboardButtonRequestChat {
    #[must_use]
    pub fn chat_is_forum_option(self, val: Option<bool>) -> Self {
        Self {
            chat_is_forum: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_has_username_option(self, val: Option<bool>) -> Self {
        Self {
            chat_has_username: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_is_created_option(self, val: Option<bool>) -> Self {
        Self {
            chat_is_created: val,
            ..self
        }
    }

    #[must_use]
    pub fn user_administrator_rights_option(
        self,
        val: Option<impl Into<ChatAdministratorRights>>,
    ) -> Self {
        Self {
            user_administrator_rights: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn bot_administrator_rights_option(
        self,
        val: Option<impl Into<ChatAdministratorRights>>,
    ) -> Self {
        Self {
            bot_administrator_rights: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn bot_is_member_option(self, val: Option<bool>) -> Self {
        Self {
            bot_is_member: val,
            ..self
        }
    }
}
