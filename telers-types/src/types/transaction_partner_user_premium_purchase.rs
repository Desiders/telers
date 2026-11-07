/// This object represents a/an premium purchase transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `premium_purchase`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
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
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// Number of months the gifted Telegram Premium subscription will be active for; for `premium_purchase` transactions only
    #[must_use]
    pub fn premium_subscription_duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.premium_subscription_duration = val.into();
        self
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for TransactionPartnerUserPremiumPurchase {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "TransactionPartnerUserPremiumPurchase",
            fields: &["user", "premium_subscription_duration"],
            specific: &["premium_subscription_duration"],
            sequence: "struct TransactionPartnerUserPremiumPurchase with 2 elements",
        };
        let (common, (value_premium_subscription_duration,)) =
            crate::deserialization::chat_member::read_1::<D, i64>(deserializer, meta)?;
        Ok(Self {
            user: common.user,
            premium_subscription_duration: value_premium_subscription_duration,
        })
    }
}
