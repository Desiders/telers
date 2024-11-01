use serde::{Deserialize, Serialize};

/// Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api#paid-broadcasts)
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramapi>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionPartnerTelegramApi {
    /// The number of successful requests that exceeded regular limits and were therefore billed
    pub request_count: i64,
}
