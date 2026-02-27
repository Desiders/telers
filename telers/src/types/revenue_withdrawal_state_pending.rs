use serde::{Deserialize, Serialize};
/// The withdrawal is in progress.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatepending>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
