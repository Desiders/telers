use serde::{Deserialize, Serialize};
/// Contains a list of Telegram Star transactions.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransactions>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Box<[crate::types::StarTransaction]>,
}
impl StarTransactions {
    /// Creates a new `StarTransactions`.
    ///
    /// # Arguments
    /// * `transactions` - The list of transactions
    #[must_use]
    pub fn new<T0Item: Into<crate::types::StarTransaction>, T0: IntoIterator<Item = T0Item>>(
        transactions: T0,
    ) -> Self {
        Self {
            transactions: transactions.into_iter().map(Into::into).collect(),
        }
    }

    /// The list of transactions
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn transactions<T: Into<Box<[crate::types::StarTransaction]>>>(mut self, val: T) -> Self {
        self.transactions = self
            .transactions
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of transactions
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn transaction<T: Into<crate::types::StarTransaction>>(mut self, val: T) -> Self {
        self.transactions = self
            .transactions
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
