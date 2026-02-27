use crate::client::Bot;
use serde::Serialize;
/// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#edituserstarsubscription>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditUserStarSubscription {
    /// Identifier of the user whose subscription will be edited
    pub user_id: i64,
    /// Telegram payment identifier for the subscription
    pub telegram_payment_charge_id: Box<str>,
    /// Pass `true` to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass `false` to allow the user to re-enable a subscription that was previously canceled by the bot.
    pub is_canceled: bool,
}
impl EditUserStarSubscription {
    /// Creates a new `EditUserStarSubscription`.
    ///
    /// # Arguments
    /// * `user_id` - Identifier of the user whose subscription will be edited
    /// * `telegram_payment_charge_id` - Telegram payment identifier for the subscription
    /// * `is_canceled` - Pass `true` to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass `false` to allow the user to re-enable a subscription that was previously canceled by the bot.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>, T2: Into<bool>>(
        user_id: T0,
        telegram_payment_charge_id: T1,
        is_canceled: T2,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled: is_canceled.into(),
        }
    }

    /// Identifier of the user whose subscription will be edited
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Telegram payment identifier for the subscription
    #[must_use]
    pub fn telegram_payment_charge_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.telegram_payment_charge_id = val.into();
        this
    }

    /// Pass `true` to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass `false` to allow the user to re-enable a subscription that was previously canceled by the bot.
    #[must_use]
    pub fn is_canceled<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_canceled = val.into();
        this
    }
}
impl super::TelegramMethod for EditUserStarSubscription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editUserStarSubscription", self, None)
    }
}
