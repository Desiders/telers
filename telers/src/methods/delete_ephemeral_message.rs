use crate::client::Bot;
use serde::Serialize;
/// Use this method to delete an ephemeral message. Note that it is not guaranteed that the user will receive the message deletion event, especially if they are offline. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deleteephemeralmessage>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteEphemeralMessage {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the user who received the message
    pub receiver_user_id: i64,
    /// Identifier of the ephemeral message to delete
    pub ephemeral_message_id: i64,
}
impl DeleteEphemeralMessage {
    /// Creates a new `DeleteEphemeralMessage`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `receiver_user_id` - Identifier of the user who received the message
    /// * `ephemeral_message_id` - Identifier of the ephemeral message to delete
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>, T2: Into<i64>>(
        chat_id: T0,
        receiver_user_id: T1,
        ephemeral_message_id: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            receiver_user_id: receiver_user_id.into(),
            ephemeral_message_id: ephemeral_message_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the user who received the message
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = val.into();
        self
    }

    /// Identifier of the ephemeral message to delete
    #[must_use]
    pub fn ephemeral_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.ephemeral_message_id = val.into();
        self
    }
}
impl super::TelegramMethod for DeleteEphemeralMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteEphemeralMessage", self, None)
    }
}
