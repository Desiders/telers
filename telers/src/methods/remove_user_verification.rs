use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot
/// # Documentation
/// <https://core.telegram.org/bots/api#removeuserverification>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RemoveUserVerification {
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl RemoveUserVerification {
    #[must_use]
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self { user_id: val }
    }
}

impl TelegramMethod for RemoveUserVerification {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("removeUserVerification", self, None)
    }
}

impl AsRef<RemoveUserVerification> for RemoveUserVerification {
    fn as_ref(&self) -> &Self {
        self
    }
}
