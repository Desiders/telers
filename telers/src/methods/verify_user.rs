use crate::client::Bot;
use serde::Serialize;
/// Verifies a user on behalf of the organization which is represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#verifyuser>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct VerifyUser {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<Box<str>>,
}
impl VerifyUser {
    /// Creates a new `VerifyUser`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            custom_description: None,
        }
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_description = Some(val.into());
        self
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.custom_description = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for VerifyUser {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("verifyUser", self, None)
    }
}
