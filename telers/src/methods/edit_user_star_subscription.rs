use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars
/// # Documentation
/// <https://core.telegram.org/bots/api#edituserstarsubscription>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditUserStarSubscription {
    /// Identifier of the user whose subscription will be edited
    pub user_id: i64,
    /// Telegram payment identifier for the subscription
    pub telegram_payment_charge_id: String,
    /// Pass `true` to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass `false` to allow the user to re-enable a subscription that was previously canceled by the bot.
    pub is_canceled: bool,
}

impl EditUserStarSubscription {
    #[must_use]
    pub fn new(
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> Self {
        Self {
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled,
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
    pub fn telegram_payment_charge_id(self, val: impl Into<String>) -> Self {
        Self {
            telegram_payment_charge_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn is_canceled(self, val: bool) -> Self {
        Self {
            is_canceled: val,
            ..self
        }
    }
}

impl TelegramMethod for EditUserStarSubscription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editUserStarSubscription", self, None)
    }
}

impl AsRef<EditUserStarSubscription> for EditUserStarSubscription {
    fn as_ref(&self) -> &Self {
        self
    }
}
