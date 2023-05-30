use super::ChatAdministratorRights;

use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a suitable chat. The identifier of the selected chat will be shared with the bot when the corresponding button is pressed. [`More about requesting chats`](https://core.telegram.org/bots/features#chat-and-user-selection)
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestchat>
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the [`UserShared`](crate::types::UserShared) object. Must be unique within the message
    pub request_id: i64,
    /// Pass `True` to request a channel chat, pass `False` to request a group or a supergroup chat.
    pub chat_is_channel: bool,
    /// Pass `True` to request a forum supergroup, pass `False` to request a non-forum chat. If not specified, no additional restrictions are applied.
    pub chat_is_forum: Option<bool>,
    /// Pass `True` to request a supergroup or a channel with a username, pass `False` to request a chat without a username. If not specified, no additional restrictions are applied.
    pub chat_has_username: Option<bool>,
    /// Pass `True` to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    pub chat_is_created: Option<bool>,
    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of `bot_administrator_rights`. If not specified, no additional restrictions are applied.
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of `user_administrator_rights`. If not specified, no additional restrictions are applied.
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass `True` to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    pub bot_is_member: Option<bool>,
}

impl KeyboardButtonRequestChat {
    #[must_use]
    pub fn new(request_id: i64, chat_is_channel: bool) -> Self {
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
    pub fn request_id(self, val: i64) -> Self {
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
    pub fn user_administrator_rights(self, val: ChatAdministratorRights) -> Self {
        Self {
            user_administrator_rights: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn bot_administrator_rights(self, val: ChatAdministratorRights) -> Self {
        Self {
            bot_administrator_rights: Some(val),
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
