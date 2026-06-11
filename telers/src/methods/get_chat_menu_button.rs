use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [`crate::types::MenuButton`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmenubutton>
/// # Returns
/// - `crate::types::MenuButton`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}
impl GetChatMenuButton {
    /// Creates a new `GetChatMenuButton`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chat_id: None,
        }
    }

    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be returned.
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Unique identifier for the target private chat. If not specified, the bot's default menu button will be returned.
    #[must_use]
    pub fn chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }
}
impl Default for GetChatMenuButton {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetChatMenuButton {
    type Method = Self;
    type Return = crate::types::MenuButton;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatMenuButton", self, None)
    }
}
