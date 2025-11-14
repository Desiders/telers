use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Refunds a successful payment in [`Telegram Stars`](https://t.me/BotNews/90)
/// # Documentation
/// <https://core.telegram.org/bots/api#refundstarpayment>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RefundStarPayment {
    /// Identifier of the user whose payment will be refunded
    pub user_id: i64,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
}

impl RefundStarPayment {
    #[must_use]
    pub fn new(user_id: i64, telegram_payment_charge_id: impl Into<String>) -> Self {
        Self {
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
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
}

impl TelegramMethod for RefundStarPayment {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("refundStarPayment", self, None)
    }
}

impl AsRef<RefundStarPayment> for RefundStarPayment {
    fn as_ref(&self) -> &Self {
        self
    }
}
