use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::BotCommandScope};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api#determining-list-of-commands) will be shown to affected users.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletemycommands>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct DeleteMyCommands {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`BotCommandScopeDefault`](crate::types::BotCommandScopeDefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands.
    pub language_code: Option<String>,
}

impl DeleteMyCommands {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn scope(mut self, val: BotCommandScope) -> Self {
        self.scope = Some(val);
        self
    }

    #[must_use]
    pub fn language_code<T: Into<String>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }
}

impl TelegramMethod for DeleteMyCommands {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteMyCommands", self, None)
    }
}
