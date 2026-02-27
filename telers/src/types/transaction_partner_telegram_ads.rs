use serde::{Deserialize, Serialize};
/// Describes a withdrawal transaction to the Telegram Ads platform.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramads>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramAds {}
impl TransactionPartnerTelegramAds {
    /// Creates a new `TransactionPartnerTelegramAds`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for TransactionPartnerTelegramAds {
    fn default() -> Self {
        Self::new()
    }
}
