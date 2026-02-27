use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current bot short description for the given user language. Returns [`BotShortDescription`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmyshortdescription>
/// # Returns
/// - `crate::types::BotShortDescription`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyShortDescription {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl GetMyShortDescription {
    /// Creates a new `GetMyShortDescription`.
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
impl Default for GetMyShortDescription {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyShortDescription {
    type Method = Self;
    type Return = crate::types::BotShortDescription;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyShortDescription", self, None)
    }
}
