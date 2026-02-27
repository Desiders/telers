use serde::{Deserialize, Serialize};
/// Describes a transaction with payment for paid broadcasting.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramapi>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// The number of successful requests that exceeded regular limits and were therefore billed
    pub request_count: i64,
}
impl TransactionPartnerTelegramApi {
    /// Creates a new `TransactionPartnerTelegramApi`.
    ///
    /// # Arguments
    /// * `request_count` - The number of successful requests that exceeded regular limits and were therefore billed
    #[must_use]
    pub fn new<T0: Into<i64>>(request_count: T0) -> Self {
        Self {
            request_count: request_count.into(),
        }
    }

    /// The number of successful requests that exceeded regular limits and were therefore billed
    #[must_use]
    pub fn request_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.request_count = val.into();
        this
    }
}
