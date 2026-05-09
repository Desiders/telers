use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current bot description for the given user language. Returns [`crate::types::BotDescription`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmydescription>
/// # Returns
/// - `crate::types::BotDescription`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyDescription {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl GetMyDescription {
    /// Creates a new `GetMyDescription`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            language_code: None,
        }
    }

    /// A two-letter ISO 639-1 language code or an empty string
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// A two-letter ISO 639-1 language code or an empty string
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language_code = val.map(Into::into);
        self
    }
}
impl Default for GetMyDescription {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyDescription {
    type Method = Self;
    type Return = crate::types::BotDescription;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyDescription", self, None)
    }
}
