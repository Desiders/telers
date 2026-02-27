use serde::{Deserialize, Serialize};
/// Describes a transaction with an unknown source or recipient.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerother>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerOther {}
impl TransactionPartnerOther {
    /// Creates a new `TransactionPartnerOther`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for TransactionPartnerOther {
    fn default() -> Self {
        Self::new()
    }
}
