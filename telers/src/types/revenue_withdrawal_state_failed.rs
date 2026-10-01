/// The withdrawal failed and the transaction was refunded.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatefailed>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct RevenueWithdrawalStateFailed {}
impl RevenueWithdrawalStateFailed {
    /// Creates a new `RevenueWithdrawalStateFailed`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for RevenueWithdrawalStateFailed {
    fn default() -> Self {
        Self::new()
    }
}
