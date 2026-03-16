use serde::{Deserialize, Serialize};
/// Describes a withdrawal transaction with Fragment.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerfragment>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerFragment {
    /// State of the transaction if the transaction is outgoing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<crate::types::RevenueWithdrawalState>,
}
impl TransactionPartnerFragment {
    /// Creates a new `TransactionPartnerFragment`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            withdrawal_state: None,
        }
    }

    /// State of the transaction if the transaction is outgoing
    #[must_use]
    pub fn withdrawal_state<T: Into<crate::types::RevenueWithdrawalState>>(self, val: T) -> Self {
        let mut this = self;
        this.withdrawal_state = Some(val.into());
        this
    }

    /// State of the transaction if the transaction is outgoing
    #[must_use]
    pub fn withdrawal_state_option<T: Into<crate::types::RevenueWithdrawalState>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.withdrawal_state = val.map(Into::into);
        this
    }
}
impl Default for TransactionPartnerFragment {
    fn default() -> Self {
        Self::new()
    }
}
