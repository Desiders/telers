use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::{BotCommandScope, BotCommand}};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current list of the bot's commands for the given scope and user language.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmycommands>
/// # Returns
/// Returns an Array of [`BotCommand`] objects. If commands aren't set, an empty list is returned.
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetMyCommands {
    /// A JSON-serialized object, describing scope of users. Defaults to [`BotCommandScopeDefault`](crate::types::BotCommandScopeDefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code or an empty string.
    pub language_code: Option<String>,
}

impl GetMyCommands {
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

impl TelegramMethod for GetMyCommands {
    type Method = Self;
    type Return = Vec<BotCommand>;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getMyCommands", self, None)
    }
}
