use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a suitable user. The identifier of the selected user will be shared with the bot when the corresponding button is pressed. [`More about requesting users`](https://core.telegram.org/bots/features#chat-and-user-selection)
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestuser>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeyboardButtonRequestUser {
    /// Signed 32-bit identifier of the request, which will be received back in the [`UserShared`](crate::types::UserShared) object. Must be unique within the message
    pub request_id: i64,
    /// Pass `true` to request a bot, pass `False` to request a regular user. If not specified, no additional restrictions are applied.
    pub user_is_bot: Option<bool>,
    /// Pass `true` to request a premium user, pass `False` to request a non-premium user. If not specified, no additional restrictions are applied.
    pub user_is_premium: Option<bool>,
}

impl KeyboardButtonRequestUser {
    #[must_use]
    pub fn new(request_id: i64) -> Self {
        Self {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
        }
    }

    #[must_use]
    pub fn request_id(self, val: i64) -> Self {
        Self {
            request_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn user_is_bot(self, val: bool) -> Self {
        Self {
            user_is_bot: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn user_is_premium(self, val: bool) -> Self {
        Self {
            user_is_premium: Some(val),
            ..self
        }
    }
}

impl KeyboardButtonRequestUser {
    #[must_use]
    pub fn user_is_bot_option(self, val: Option<bool>) -> Self {
        Self {
            user_is_bot: val,
            ..self
        }
    }

    #[must_use]
    pub fn user_is_premium_option(self, val: Option<bool>) -> Self {
        Self {
            user_is_premium: val,
            ..self
        }
    }
}
