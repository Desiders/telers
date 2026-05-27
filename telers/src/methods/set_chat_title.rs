use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchattitle>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// New chat title, 1-128 characters
    pub title: Box<str>,
}
impl SetChatTitle {
    /// Creates a new `SetChatTitle`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `title` - New chat title, 1-128 characters
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        title: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// New chat title, 1-128 characters
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }
}
impl super::TelegramMethod for SetChatTitle {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatTitle", self, None)
    }
}
