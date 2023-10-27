use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmyshortdescription>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetMyShortDescription {
    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    pub short_description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    pub language_code: Option<String>,
}

impl SetMyShortDescription {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn short_description(self, val: impl Into<String>) -> Self {
        Self {
            short_description: Some(val.into()),
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

impl SetMyShortDescription {
    #[must_use]
    pub fn short_description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            short_description: val.map(Into::into),
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

impl TelegramMethod for SetMyShortDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMyShortDescription", self, None)
    }
}

impl AsRef<SetMyShortDescription> for SetMyShortDescription {
    fn as_ref(&self) -> &Self {
        self
    }
}
