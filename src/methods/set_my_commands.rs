use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{BotCommand, BotCommandScope},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmycommands>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`BotCommandScopeDefault`](crate::types::BotCommandScopeDefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands.
    pub language_code: Option<String>,
}

impl SetMyCommands {
    #[must_use]
    pub fn new(commands: impl IntoIterator<Item = BotCommand>) -> Self {
        Self {
            commands: commands.into_iter().collect(),
            scope: None,
            language_code: None,
        }
    }

    #[must_use]
    pub fn command(self, val: BotCommand) -> Self {
        Self {
            commands: self.commands.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn commands(self, val: impl IntoIterator<Item = BotCommand>) -> Self {
        Self {
            commands: self.commands.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn scope(self, val: impl Into<BotCommandScope>) -> Self {
        Self {
            scope: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn language_code(self, val: impl Into<String>) -> Self {
        Self {
            language_code: Some(val.into()),
            ..self
        }
    }
}

impl SetMyCommands {
    #[must_use]
    pub fn scope_option(self, val: Option<impl Into<BotCommandScope>>) -> Self {
        Self {
            scope: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn language_code_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            language_code: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetMyCommands {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMyCommands", self, None)
    }
}

impl AsRef<SetMyCommands> for SetMyCommands {
    fn as_ref(&self) -> &Self {
        self
    }
}
