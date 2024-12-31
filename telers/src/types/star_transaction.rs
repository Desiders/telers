use super::TransactionPartner;

use serde::{Deserialize, Serialize};

/// Describes a Telegram Star transaction.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StarTransaction {
    Source(Source),
    Receiver(Receiver),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Source {
    /// Unique identifier of the transaction. Coincides with the identifer of the original transaction for refund transactions. Coincides with [`SuccessfulPayment`](super::SuccessfulPayment)`.telegram_payment_charge_id` for successful incoming payments from users.
    pub id: Box<str>,
    /// Number of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    pub nanostar_amount: Option<i64>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal)
    pub source: TransactionPartner,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Receiver {
    /// Unique identifier of the transaction. Coincides with the identifer of the original transaction for refund transactions. Coincides with [`SuccessfulPayment`](super::SuccessfulPayment)`.telegram_payment_charge_id` for successful incoming payments from users.
    pub id: Box<str>,
    /// Number of Telegram Stars transferred by the transaction
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    pub nanostar_amount: Option<i64>,
    /// Date the transaction was created in Unix time
    pub date: i64,
    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal)
    pub receiver: TransactionPartner,
}

impl StarTransaction {
    #[must_use]
    pub const fn id(&self) -> &str {
        match self {
            StarTransaction::Source(Source { id, .. })
            | StarTransaction::Receiver(Receiver { id, .. }) => id,
        }
    }

    #[must_use]
    pub const fn amount(&self) -> i64 {
        match self {
            StarTransaction::Source(Source { amount, .. })
            | StarTransaction::Receiver(Receiver { amount, .. }) => *amount,
        }
    }

    #[must_use]
    pub const fn nanostar_amount(&self) -> Option<i64> {
        match self {
            StarTransaction::Source(Source {
                nanostar_amount, ..
            })
            | StarTransaction::Receiver(Receiver {
                nanostar_amount, ..
            }) => *nanostar_amount,
        }
    }

    #[must_use]
    pub const fn date(&self) -> i64 {
        match self {
            StarTransaction::Source(Source { date, .. })
            | StarTransaction::Receiver(Receiver { date, .. }) => *date,
        }
    }

    #[must_use]
    pub const fn source(&self) -> Option<&TransactionPartner> {
        if let Self::Source(Source { ref source, .. }) = self {
            Some(source)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn receiver(&self) -> Option<&TransactionPartner> {
        if let Self::Receiver(Receiver { ref receiver, .. }) = self {
            Some(receiver)
        } else {
            None
        }
    }
}
