#[cfg(not(feature = "deser"))]
use crate::deserialization::transaction_partner_user as decoder;

/// This object represents a/an business account transfer transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `business_account_transfer`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct TransactionPartnerUserBusinessAccountTransfer {
    /// Information about the user
    pub user: Box<crate::types::User>,
}

impl TransactionPartnerUserBusinessAccountTransfer {
    /// Creates a new `TransactionPartnerUserBusinessAccountTransfer`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}

#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for TransactionPartnerUserBusinessAccountTransfer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded =
            decoder::deserialize_common(deserializer, &decoder::BUSINESS_ACCOUNT_TRANSFER)?;
        Ok(Self {
            user: decoded.user,
        })
    }
}
