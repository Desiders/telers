use crate::client::Bot;
use serde::Serialize;
/// Removes verification from a chat that is currently verified on behalf of the organization represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#removechatverification>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RemoveChatVerification {
    /// Unique identifier for the target chat or username of the target bot or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
}
impl RemoveChatVerification {
    /// Creates a new `RemoveChatVerification`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot or channel in the format @username
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target bot or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}
impl super::TelegramMethod for RemoveChatVerification {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("removeChatVerification", self, None)
    }
}
