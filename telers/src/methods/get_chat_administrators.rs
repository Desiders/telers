use crate::client::Bot;
use serde::Serialize;
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [`crate::types::ChatMember`] objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatadministrators>
/// # Returns
/// - `Box<[crate::types::ChatMember]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
}
impl GetChatAdministrators {
    /// Creates a new `GetChatAdministrators`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for GetChatAdministrators {
    type Method = Self;
    type Return = Box<[crate::types::ChatMember]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatAdministrators", self, None)
    }
}
