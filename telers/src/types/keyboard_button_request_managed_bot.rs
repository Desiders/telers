use serde::{Deserialize, Serialize};
/// This object defines the parameters for the creation of a managed bot. Information about the created bot will be shared with the bot using the update `managed_bot` and a Message with the field `managed_bot_created`.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the message.
    pub request_id: i64,
    /// Suggested name for the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<Box<str>>,
    /// Suggested username for the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_username: Option<Box<str>>,
}
impl KeyboardButtonRequestManagedBot {
    /// Creates a new `KeyboardButtonRequestManagedBot`.
    ///
    /// # Arguments
    /// * `request_id` - Signed 32-bit identifier of the request. Must be unique within the message.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(request_id: T0) -> Self {
        Self {
            request_id: request_id.into(),
            suggested_name: None,
            suggested_username: None,
        }
    }

    /// Signed 32-bit identifier of the request. Must be unique within the message.
    #[must_use]
    pub fn request_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.request_id = val.into();
        self
    }

    /// Suggested name for the bot
    #[must_use]
    pub fn suggested_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.suggested_name = Some(val.into());
        self
    }

    /// Suggested name for the bot
    #[must_use]
    pub fn suggested_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.suggested_name = val.map(Into::into);
        self
    }

    /// Suggested username for the bot
    #[must_use]
    pub fn suggested_username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.suggested_username = Some(val.into());
        self
    }

    /// Suggested username for the bot
    #[must_use]
    pub fn suggested_username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.suggested_username = val.map(Into::into);
        self
    }
}
