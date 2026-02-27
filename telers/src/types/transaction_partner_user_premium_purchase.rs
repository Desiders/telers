use serde::{Deserialize, Serialize};
/// This object represents a/an premium purchase transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `premium_purchase`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerUserPremiumPurchase {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// Number of months the gifted Telegram Premium subscription will be active for; for `premium_purchase` transactions only
    pub premium_subscription_duration: i64,
}
impl TransactionPartnerUserPremiumPurchase {
    /// Creates a new `TransactionPartnerUserPremiumPurchase`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `premium_subscription_duration` - Number of months the gifted Telegram Premium subscription will be active for; for `premium_purchase` transactions only
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<i64>>(
        user: T0,
        premium_subscription_duration: T1,
    ) -> Self {
        Self {
            user: Box::new(user.into()),
            premium_subscription_duration: premium_subscription_duration.into(),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// Number of months the gifted Telegram Premium subscription will be active for; for `premium_purchase` transactions only
    #[must_use]
    pub fn premium_subscription_duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.premium_subscription_duration = val.into();
        this
    }
}
