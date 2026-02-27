use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current bot name for the given user language. Returns [`BotName`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmyname>
/// # Returns
/// - `crate::types::BotName`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyName {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl GetMyName {
    /// Creates a new `GetMyName`.
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
    pub fn language_code<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.language_code = Some(val.into());
        this
    }

    /// A two-letter ISO 639-1 language code or an empty string
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.language_code = val.map(Into::into);
        this
    }
}
impl Default for GetMyName {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyName {
    type Method = Self;
    type Return = crate::types::BotName;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyName", self, None)
    }
}
