use serde::{Deserialize, Serialize};
/// This object represents an incoming star transaction.
/// # Notes
/// This object represents a star transaction from original field `incoming`.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarTransactionIncoming {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    pub id: Box<str>,
    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<u32>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions.
    pub source: crate::types::TransactionPartner,
}
impl StarTransactionIncoming {
    /// Creates a new `StarTransactionIncoming`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    /// * `amount` - Integer amount of Telegram Stars transferred by the transaction
    /// * `date` - Date the transaction was created in Unix time
    /// * `source` - Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<crate::types::TransactionPartner>,
    >(
        id: T0,
        amount: T1,
        date: T2,
        source: T3,
    ) -> Self {
        Self {
            id: id.into(),
            amount: amount.into(),
            nanostar_amount: None,
            date: date.into(),
            source: source.into(),
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

    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[must_use]
    pub fn nanostar_amount<T: Into<u32>>(mut self, val: T) -> Self {
        self.nanostar_amount = Some(val.into());
        self
    }

    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[must_use]
    pub fn nanostar_amount_option<T: Into<u32>>(mut self, val: Option<T>) -> Self {
        self.nanostar_amount = val.map(Into::into);
        self
    }

    /// Date the transaction was created in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions.
    #[must_use]
    pub fn source<T: Into<crate::types::TransactionPartner>>(mut self, val: T) -> Self {
        self.source = val.into();
        self
    }
}
