use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the number of members in a chat. Returns Int on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmembercount>
/// # Returns
/// - `i64`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
}
impl GetChatMemberCount {
    /// Creates a new `GetChatMemberCount`.
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
impl super::TelegramMethod for GetChatMemberCount {
    type Method = Self;
    type Return = i64;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatMemberCount", self, None)
    }
}
