use serde::{Deserialize, Serialize};
/// # Documentation
/// <https://core.telegram.org/bots/api#ephemeralmessageparameters>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    pub receiver_user_id: i64,
    /// Identifier of the callback query which triggered the message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<Box<str>>,
    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_callback_query_message: Option<bool>,
}
impl EphemeralMessageParameters {
    /// Creates a new `EphemeralMessageParameters`.
    ///
    /// # Arguments
    /// * `receiver_user_id` - Identifier of the user who will receive the message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(receiver_user_id: T0) -> Self {
        Self {
            receiver_user_id: receiver_user_id.into(),
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Identifier of the user who will receive the message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = val.into();
        self
    }

    /// Identifier of the callback query which triggered the message, if any
    #[must_use]
    pub fn callback_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_query_id = Some(val.into());
        self
    }

    /// Identifier of the callback query which triggered the message, if any
    #[must_use]
    pub fn callback_query_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_query_id = val.map(Into::into);
        self
    }

    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[must_use]
    pub fn replace_callback_query_message<T: Into<bool>>(mut self, val: T) -> Self {
        self.replace_callback_query_message = Some(val.into());
        self
    }

    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[must_use]
    pub fn replace_callback_query_message_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.replace_callback_query_message = val.map(Into::into);
        self
    }
}
