use crate::client::Bot;
use serde::Serialize;
/// Use this method to get a list of administrators in a chat. Returns an Array of [`crate::types::ChatMember`] objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatadministrators>
/// # Returns
/// - `Box<[crate::types::ChatMember]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Pass `true` to additionally receive all bots that are administrators of the chat. By default, bots other than the current bot are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_bots: Option<bool>,
}
impl GetChatAdministrators {
    /// Creates a new `GetChatAdministrators`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup or channel in the format @username
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
            return_bots: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Pass `true` to additionally receive all bots that are administrators of the chat. By default, bots other than the current bot are omitted.
    #[must_use]
    pub fn return_bots<T: Into<bool>>(mut self, val: T) -> Self {
        self.return_bots = Some(val.into());
        self
    }

    /// Pass `true` to additionally receive all bots that are administrators of the chat. By default, bots other than the current bot are omitted.
    #[must_use]
    pub fn return_bots_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.return_bots = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for GetChatAdministrators {
    type Method = Self;
    type Return = Box<[crate::types::ChatMember]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatAdministrators", self, None)
    }
}
