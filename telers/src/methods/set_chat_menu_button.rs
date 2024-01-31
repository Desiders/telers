use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::MenuButton};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the bot's menu button in a private chat, or the default menu button.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatmenubutton>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed.
    pub chat_id: i64,
    /// A JSON-serialized object for the bot's new menu button. Defaults to [`MenuButtonDefault`](crate::types::MenuButtonDefault).
    pub menu_button: Option<MenuButton>,
}

impl SetChatMenuButton {
    #[must_use]
    pub fn new(chat_id: i64) -> Self {
        Self {
            chat_id,
            menu_button: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: i64) -> Self {
        Self {
            chat_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn menu_button(self, val: impl Into<MenuButton>) -> Self {
        Self {
            menu_button: Some(val.into()),
            ..self
        }
    }
}

impl SetChatMenuButton {
    #[must_use]
    pub fn menu_button_option(self, val: Option<impl Into<MenuButton>>) -> Self {
        Self {
            menu_button: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetChatMenuButton {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatMenuButton", self, None)
    }
}

impl AsRef<SetChatMenuButton> for SetChatMenuButton {
    fn as_ref(&self) -> &Self {
        self
    }
}
