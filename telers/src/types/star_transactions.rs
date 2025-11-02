use super::StarTransaction;

use serde::Deserialize;

/// Contains a list of Telegram Star transactions.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransactions>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Box<[StarTransaction]>,
}
