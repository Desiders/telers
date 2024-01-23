use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{BotCommand, BotCommandScope},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current list of the bot's commands for the given scope and user language.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmycommands>
/// # Returns
/// Returns an Array of [`BotCommand`] objects. If commands aren't set, an empty list is returned.
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
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

impl GetMyCommands {
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

impl TelegramMethod for GetMyCommands {
    type Method = Self;
    type Return = Vec<BotCommand>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getMyCommands", self, None)
    }
}

impl AsRef<GetMyCommands> for GetMyCommands {
    fn as_ref(&self) -> &Self {
        self
    }
}
