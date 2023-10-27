use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::BotDescription};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current bot description for the given user language.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmydescription>
/// # Returns
/// Returns [`BotDescription`] on success
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetMyDescription {
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}

impl GetMyDescription {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn language_code(self, val: impl Into<String>) -> Self {
        Self {
            language_code: Some(val.into()),
        }
    }
}

impl GetMyDescription {
    #[must_use]
    pub fn language_code_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            language_code: val.map(Into::into),
        }
    }
}

impl TelegramMethod for GetMyDescription {
    type Method = Self;
    type Return = BotDescription;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getMyDescription", self, None)
    }
}

impl AsRef<GetMyDescription> for GetMyDescription {
    fn as_ref(&self) -> &Self {
        self
    }
}
