use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::MenuButton};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmenubutton>
/// # Returns
/// Returns [`MenuButton`] on success
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    pub chat_id: Option<i64>,
}

impl GetChatMenuButton {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn chat_id(self, val: i64) -> Self {
        Self { chat_id: Some(val) }
    }
}

impl GetChatMenuButton {
    #[must_use]
    pub fn chat_id_option(self, val: i64) -> Self {
        Self { chat_id: Some(val) }
    }
}

impl TelegramMethod for GetChatMenuButton {
    type Method = Self;
    type Return = MenuButton;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getChatMenuButton", self, None)
    }
}
