use serde::{Deserialize, Serialize};
/// This object represents an outgoing star transaction.
/// # Notes
/// This object represents a star transaction from original field `outgoing`.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarTransactionOutgoing {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    pub id: Box<str>,
    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<u32>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    pub receiver: crate::types::TransactionPartner,
}
impl StarTransactionOutgoing {
    /// Creates a new `StarTransactionOutgoing`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    /// * `amount` - Integer amount of Telegram Stars transferred by the transaction
    /// * `date` - Date the transaction was created in Unix time
    /// * `receiver` - Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
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
        receiver: T3,
    ) -> Self {
        Self {
            id: id.into(),
            amount: amount.into(),
            nanostar_amount: None,
            date: date.into(),
            receiver: receiver.into(),
        }
    }

    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`crate::types::SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Integer amount of Telegram Stars transferred by the transaction
    #[must_use]
    pub fn amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.amount = val.into();
        this
    }

    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[must_use]
    pub fn nanostar_amount<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.nanostar_amount = Some(val.into());
        this
    }

    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[must_use]
    pub fn nanostar_amount_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.nanostar_amount = val.map(Into::into);
        this
    }

    /// Date the transaction was created in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[must_use]
    pub fn receiver<T: Into<crate::types::TransactionPartner>>(self, val: T) -> Self {
        let mut this = self;
        this.receiver = val.into();
        this
    }
}
