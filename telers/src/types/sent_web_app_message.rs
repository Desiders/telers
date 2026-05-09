use serde::{Deserialize, Serialize};
/// Describes an inline message sent by a Web App on behalf of a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#sentwebappmessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
}
impl SentWebAppMessage {
    /// Creates a new `SentWebAppMessage`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inline_message_id: None,
        }
    }

    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }
}
impl Default for SentWebAppMessage {
    fn default() -> Self {
        Self::new()
    }
}
