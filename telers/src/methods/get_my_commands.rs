use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [`crate::types::BotCommand`] objects. If commands aren't set, an empty list is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmycommands>
/// # Returns
/// - `Box<[crate::types::BotCommand]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyCommands {
    /// A JSON-serialized object, describing scope of users. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl GetMyCommands {
    /// Creates a new `GetMyCommands`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }

    /// A JSON-serialized object, describing scope of users. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[must_use]
    pub fn scope<T: Into<crate::types::BotCommandScope>>(mut self, val: T) -> Self {
        self.scope = Some(val.into());
        self
    }

    /// A JSON-serialized object, describing scope of users. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[must_use]
    pub fn scope_option<T: Into<crate::types::BotCommandScope>>(mut self, val: Option<T>) -> Self {
        self.scope = val.map(Into::into);
        self
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
impl Default for GetMyCommands {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyCommands {
    type Method = Self;
    type Return = Box<[crate::types::BotCommand]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyCommands", self, None)
    }
}
