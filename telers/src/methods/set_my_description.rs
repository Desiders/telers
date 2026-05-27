use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmydescription>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyDescription {
    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl SetMyDescription {
    /// Creates a new `SetMyDescription`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            language_code: None,
        }
    }

    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.description = val.map(Into::into);
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language_code = val.map(Into::into);
        self
    }
}
impl Default for SetMyDescription {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetMyDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyDescription", self, None)
    }
}
