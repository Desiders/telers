use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmyshortdescription>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyShortDescription {
    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<Box<str>>,
    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl SetMyShortDescription {
    /// Creates a new `SetMyShortDescription`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            short_description: None,
            language_code: None,
        }
    }

    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[must_use]
    pub fn short_description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.short_description = Some(val.into());
        this
    }

    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[must_use]
    pub fn short_description_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.short_description = val.map(Into::into);
        this
    }

    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.language_code = Some(val.into());
        this
    }

    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.language_code = val.map(Into::into);
        this
    }
}
impl Default for SetMyShortDescription {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetMyShortDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyShortDescription", self, None)
    }
}
