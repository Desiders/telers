/// The withdrawal is in progress.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatepending>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct RevenueWithdrawalStatePending {}
impl RevenueWithdrawalStatePending {
    /// Creates a new `RevenueWithdrawalStatePending`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for RevenueWithdrawalStatePending {
    fn default() -> Self {
        Self::new()
    }
}
