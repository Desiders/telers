use serde::{Deserialize, Serialize};
/// Describes the affiliate program that issued the affiliate commission received via this transaction.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneraffiliateprogram>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// Information about the bot that sponsored the affiliate program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<Box<crate::types::User>>,
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: i64,
}
impl TransactionPartnerAffiliateProgram {
    /// Creates a new `TransactionPartnerAffiliateProgram`.
    ///
    /// # Arguments
    /// * `commission_per_mille` - The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(commission_per_mille: T0) -> Self {
        Self {
            sponsor_user: None,
            commission_per_mille: commission_per_mille.into(),
        }
    }

    /// Information about the bot that sponsored the affiliate program
    #[must_use]
    pub fn sponsor_user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.sponsor_user = Some(Box::new(val.into()));
        this
    }

    /// Information about the bot that sponsored the affiliate program
    #[must_use]
    pub fn sponsor_user_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sponsor_user = val.map(|val| Box::new(val.into()));
        this
    }

    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    #[must_use]
    pub fn commission_per_mille<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.commission_per_mille = val.into();
        this
    }
}
