use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmydescription>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetMyDescription {
    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    pub language_code: Option<String>,
}

impl SetMyDescription {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: Some(val.into()),
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

impl SetMyDescription {
    #[must_use]
    pub fn description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            description: val.map(Into::into),
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

impl TelegramMethod for SetMyDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMyDescription", self, None)
    }
}

impl AsRef<SetMyDescription> for SetMyDescription {
    fn as_ref(&self) -> &Self {
        self
    }
}
