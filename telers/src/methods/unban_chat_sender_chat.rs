use crate::client::Bot;
use serde::Serialize;
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unbanchatsenderchat>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
impl UnbanChatSenderChat {
    /// Creates a new `UnbanChatSenderChat`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `sender_chat_id` - Unique identifier of the target sender chat
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        sender_chat_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id: sender_chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target sender chat
    #[must_use]
    pub fn sender_chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.sender_chat_id = val.into();
        self
    }
}
impl super::TelegramMethod for UnbanChatSenderChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unbanChatSenderChat", self, None)
    }
}
