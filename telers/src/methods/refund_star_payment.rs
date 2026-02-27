use crate::client::Bot;
use serde::Serialize;
/// Refunds a successful payment in Telegram Stars. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#refundstarpayment>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RefundStarPayment {
    /// Identifier of the user whose payment will be refunded
    pub user_id: i64,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: Box<str>,
}
impl RefundStarPayment {
    /// Creates a new `RefundStarPayment`.
    ///
    /// # Arguments
    /// * `user_id` - Identifier of the user whose payment will be refunded
    /// * `telegram_payment_charge_id` - Telegram payment identifier
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(
        user_id: T0,
        telegram_payment_charge_id: T1,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
        }
    }

    /// Identifier of the user whose payment will be refunded
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Telegram payment identifier
    #[must_use]
    pub fn telegram_payment_charge_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.telegram_payment_charge_id = val.into();
        this
    }
}
impl super::TelegramMethod for RefundStarPayment {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("refundStarPayment", self, None)
    }
}
