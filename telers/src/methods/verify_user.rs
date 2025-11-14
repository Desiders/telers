use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot
/// # Documentation
/// <https://core.telegram.org/bots/api#verifyuser>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct VerifyUser {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    pub custom_description: Option<String>,
}

impl VerifyUser {
    #[must_use]
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            custom_description: None,
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
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

impl VerifyUser {
    #[must_use]
    pub fn custom_description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            custom_description: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for VerifyUser {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("verifyUser", self, None)
    }
}

impl AsRef<VerifyUser> for VerifyUser {
    fn as_ref(&self) -> &Self {
        self
    }
}
