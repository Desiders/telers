/// Describes a withdrawal transaction to the Telegram Ads platform.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramads>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
