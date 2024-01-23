use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request suitable users. The identifiers of the selected users will be shared with the bot when the corresponding button is pressed. [`More about requesting users`](https://core.telegram.org/bots/features#chat-and-user-selection)
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestusers>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the [`UserShared`](crate::types::UserShared) object. Must be unique within the message
    pub request_id: i64,
    /// Pass `true` to request bots, pass `false` to request regular users. If not specified, no additional restrictions are applied.
    pub user_is_bot: Option<bool>,
    /// Pass `true` to request premium users, pass `false` to request non-premium users. If not specified, no additional restrictions are applied.
    pub user_is_premium: Option<bool>,
    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    pub max_quantity: Option<i64>,
}

impl KeyboardButtonRequestUsers {
    #[must_use]
    pub const fn new(request_id: i64) -> Self {
        Self {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
            max_quantity: None,
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

    #[must_use]
    pub fn max_quantity(self, val: i64) -> Self {
        Self {
            max_quantity: Some(val),
            ..self
        }
    }
}

impl KeyboardButtonRequestUsers {
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

    #[must_use]
    pub fn max_quantity_option(self, val: Option<i64>) -> Self {
        Self {
            max_quantity: val,
            ..self
        }
    }
}
