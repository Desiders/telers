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
/// Returns `True` on success.
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
    pub fn new(commands: Vec<BotCommand>) -> Self {
        Self {
            commands,
            scope: None,
            language_code: None,
        }
    }

    #[must_use]
    pub fn commands(mut self, val: Vec<BotCommand>) -> Self {
        self.commands = val;
        self
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

impl TelegramMethod for SetMyCommands {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("setMyCommands", self, None)
    }
}
