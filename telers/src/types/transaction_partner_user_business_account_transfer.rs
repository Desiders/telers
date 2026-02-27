use serde::{Deserialize, Serialize};
/// This object represents a/an business account transfer transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `business_account_transfer`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }
}
