/// Describes a transaction with an unknown source or recipient.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerother>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
