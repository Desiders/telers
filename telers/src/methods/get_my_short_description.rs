use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::BotShortDescription};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current bot short description for the given user language.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmyshortdescription>
/// # Returns
/// Returns [`BotShortDescription`] on success
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetMyShortDescription {
    /// A two-letter ISO 639-1 language code or an empty string
    pub language_code: Option<String>,
}

impl GetMyShortDescription {
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

impl GetMyShortDescription {
    #[must_use]
    pub fn language_code_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            language_code: val.map(Into::into),
        }
    }
}

impl TelegramMethod for GetMyShortDescription {
    type Method = Self;
    type Return = BotShortDescription;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getMyShortDescription", self, None)
    }
}

impl AsRef<GetMyShortDescription> for GetMyShortDescription {
    fn as_ref(&self) -> &Self {
        self
    }
}
