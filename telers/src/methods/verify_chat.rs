use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot
/// # Documentation
/// <https://core.telegram.org/bots/api#verifychat>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct VerifyChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatIdKind,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    pub custom_description: Option<String>,
}

impl VerifyChat {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
            custom_description: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn custom_description(self, val: impl Into<String>) -> Self {
        Self {
            custom_description: Some(val.into()),
            ..self
        }
    }
}

impl VerifyChat {
    #[must_use]
    pub fn custom_description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            custom_description: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for VerifyChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("verifyChat", self, None)
    }
}

impl AsRef<VerifyChat> for VerifyChat {
    fn as_ref(&self) -> &Self {
        self
    }
}
