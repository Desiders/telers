use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatmenubutton>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to [`crate::types::MenuButtonDefault`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<crate::types::MenuButton>,
}
impl SetChatMenuButton {
    /// Creates a new `SetChatMenuButton`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chat_id: None,
            menu_button: None,
        }
    }

    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be changed.
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be changed.
    #[must_use]
    pub fn chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object for the bot's new menu button. Defaults to [`crate::types::MenuButtonDefault`].
    #[must_use]
    pub fn menu_button<T: Into<crate::types::MenuButton>>(mut self, val: T) -> Self {
        self.menu_button = Some(val.into());
        self
    }

    /// A JSON-serialized object for the bot's new menu button. Defaults to [`crate::types::MenuButtonDefault`].
    #[must_use]
    pub fn menu_button_option<T: Into<crate::types::MenuButton>>(mut self, val: Option<T>) -> Self {
        self.menu_button = val.map(Into::into);
        self
    }
}
impl Default for SetChatMenuButton {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetChatMenuButton {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatMenuButton", self, None)
    }
}
