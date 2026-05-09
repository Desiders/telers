use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the bot's name. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmyname>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyName {
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl SetMyName {
    /// Creates a new `SetMyName`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            language_code: None,
        }
    }

    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = Some(val.into());
        self
    }

    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.name = val.map(Into::into);
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language_code = val.map(Into::into);
        self
    }
}
impl Default for SetMyName {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetMyName {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyName", self, None)
    }
}
