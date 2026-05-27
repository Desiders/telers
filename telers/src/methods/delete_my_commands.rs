use crate::client::Bot;
use serde::Serialize;
/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletemycommands>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMyCommands {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl DeleteMyCommands {
    /// Creates a new `DeleteMyCommands`.
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

    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[must_use]
    pub fn scope<T: Into<crate::types::BotCommandScope>>(mut self, val: T) -> Self {
        self.scope = Some(val.into());
        self
    }

    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`crate::types::BotCommandScopeDefault`].
    #[must_use]
    pub fn scope_option<T: Into<crate::types::BotCommandScope>>(mut self, val: Option<T>) -> Self {
        self.scope = val.map(Into::into);
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language_code = val.map(Into::into);
        self
    }
}
impl Default for DeleteMyCommands {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for DeleteMyCommands {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteMyCommands", self, None)
    }
}
