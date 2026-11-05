/// Describes a withdrawal transaction with Fragment.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerfragment>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TransactionPartnerFragment {
    /// State of the transaction if the transaction is outgoing
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
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
    pub fn withdrawal_state<T: Into<crate::types::RevenueWithdrawalState>>(
        mut self,
        val: T,
    ) -> Self {
        self.withdrawal_state = Some(val.into());
        self
    }

    /// State of the transaction if the transaction is outgoing
    #[must_use]
    pub fn withdrawal_state_option<T: Into<crate::types::RevenueWithdrawalState>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.withdrawal_state = val.map(Into::into);
        self
    }
}
impl Default for TransactionPartnerFragment {
    fn default() -> Self {
        Self::new()
    }
}
