use crate::client::Bot;
use serde::Serialize;
/// Use this method for your bot to leave a group, supergroup or channel. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#leavechat>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel in the format @username. Channel direct messages chats aren't supported; leave the corresponding channel instead.
    pub chat_id: crate::types::ChatIdKind,
}
impl LeaveChat {
    /// Creates a new `LeaveChat`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup or channel in the format @username. Channel direct messages chats aren't supported; leave the corresponding channel instead.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup or channel in the format @username. Channel direct messages chats aren't supported; leave the corresponding channel instead.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}
impl super::TelegramMethod for LeaveChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("leaveChat", self, None)
    }
}
