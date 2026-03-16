use serde::{Deserialize, Serialize};
/// This object represents a/an gift purchase transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `gift_purchase`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerUserGiftPurchase {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// The gift sent to the user by the bot; for `gift_purchase` transactions only
    pub gift: Box<crate::types::Gift>,
}
impl TransactionPartnerUserGiftPurchase {
    /// Creates a new `TransactionPartnerUserGiftPurchase`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `gift` - The gift sent to the user by the bot; for `gift_purchase` transactions only
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<crate::types::Gift>>(
        user: T0,
        gift: T1,
    ) -> Self {
        Self {
            user: Box::new(user.into()),
            gift: Box::new(gift.into()),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// The gift sent to the user by the bot; for `gift_purchase` transactions only
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(self, val: T) -> Self {
        let mut this = self;
        this.gift = Box::new(val.into());
        this
    }
}
