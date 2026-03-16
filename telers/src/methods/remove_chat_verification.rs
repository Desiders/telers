use crate::client::Bot;
use serde::Serialize;
/// Removes verification from a chat that is currently verified on behalf of the organization represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#removechatverification>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RemoveChatVerification {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
}
impl RemoveChatVerification {
    /// Creates a new `RemoveChatVerification`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for RemoveChatVerification {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("removeChatVerification", self, None)
    }
}
