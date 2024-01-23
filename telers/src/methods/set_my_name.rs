use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the bot's name.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmyname>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetMyName {
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    pub name: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    pub language_code: Option<String>,
}

impl SetMyName {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: Some(val.into()),
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

impl SetMyName {
    #[must_use]
    pub fn name_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            name: val.map(Into::into),
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

impl TelegramMethod for SetMyName {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMyName", self, None)
    }
}

impl AsRef<SetMyName> for SetMyName {
    fn as_ref(&self) -> &Self {
        self
    }
}
