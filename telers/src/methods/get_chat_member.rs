use crate::client::Bot;
use serde::Serialize;
/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [`ChatMember`] object on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmember>
/// # Returns
/// - `crate::types::ChatMember`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl GetChatMember {
    /// Creates a new `GetChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    /// * `user_id` - Unique identifier of the target user
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        user_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }
}
impl super::TelegramMethod for GetChatMember {
    type Method = Self;
    type Return = crate::types::ChatMember;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatMember", self, None)
    }
}
