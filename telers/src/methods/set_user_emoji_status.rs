use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [`requestEmojiStatusAccess`](https://core.telegram.org/bots/webapps#initializing-mini-apps)
/// # Documentation
/// <https://core.telegram.org/bots/api#setuseremojistatus>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetUserEmojiStatus {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status, if any
    pub emoji_status_expiration_date: Option<i64>,
}

impl SetUserEmojiStatus {
    #[must_use]
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
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
    pub fn emoji_status_custom_emoji_id(self, val: impl Into<String>) -> Self {
        Self {
            emoji_status_custom_emoji_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn emoji_status_expiration_date(self, val: i64) -> Self {
        Self {
            emoji_status_expiration_date: Some(val),
            ..self
        }
    }
}

impl SetUserEmojiStatus {
    #[must_use]
    pub fn emoji_status_custom_emoji_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            emoji_status_custom_emoji_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn emoji_status_expiration_date_option(self, val: Option<i64>) -> Self {
        Self {
            emoji_status_expiration_date: val,
            ..self
        }
    }
}

impl TelegramMethod for SetUserEmojiStatus {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setUserEmojiStatus", self, None)
    }
}

impl AsRef<SetUserEmojiStatus> for SetUserEmojiStatus {
    fn as_ref(&self) -> &Self {
        self
    }
}
