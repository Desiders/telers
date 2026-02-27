use crate::client::Bot;
use serde::Serialize;
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#banchatsenderchat>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
impl BanChatSenderChat {
    /// Creates a new `BanChatSenderChat`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
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

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier of the target sender chat
    #[must_use]
    pub fn sender_chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for BanChatSenderChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("banChatSenderChat", self, None)
    }
}
