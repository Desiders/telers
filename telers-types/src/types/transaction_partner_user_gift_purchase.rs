#[cfg(not(feature = "deser"))]
use crate::deserialization::transaction_partner_user as decoder;

/// This object represents a/an gift purchase transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `gift_purchase`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
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
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// The gift sent to the user by the bot; for `gift_purchase` transactions only
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }
}

#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for TransactionPartnerUserGiftPurchase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded = decoder::deserialize_content(deserializer, &decoder::GIFT_PURCHASE)?;
        Ok(Self {
            user: decoded.common.user,
            gift: decoded.content,
        })
    }
}
