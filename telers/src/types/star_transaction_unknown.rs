use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::StarTransaction`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarTransactionUnknown {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    pub id: Box<str>,
    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// Date the transaction was created in Unix time
    pub date: i64,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl StarTransactionUnknown {
    /// Creates a new `StarTransactionUnknown`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    /// * `amount` - Integer amount of Telegram Stars transferred by the transaction
    /// * `date` - Date the transaction was created in Unix time
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<i64>>(
        id: T0,
        amount: T1,
        date: T2,
    ) -> Self {
        Self {
            id: id.into(),
            amount: amount.into(),
            date: date.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Integer amount of Telegram Stars transferred by the transaction
    #[must_use]
    pub fn amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }

    /// Date the transaction was created in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }
}
