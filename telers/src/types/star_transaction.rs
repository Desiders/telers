use serde::{Deserialize, Serialize};
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
/// Currently, it can be one of
/// - [`StarTransactionIncoming`]
/// - [`StarTransactionOutgoing`]
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarTransaction {
    Incoming(crate::types::StarTransactionIncoming),
    Outgoing(crate::types::StarTransactionOutgoing),
}
impl StarTransaction {
    /// Helper method for field `amount`.
    ///
    /// # Variants
    /// - `StarTransactionIncoming`. Integer amount of Telegram Stars transferred by the transaction
    /// - `StarTransactionOutgoing`. Integer amount of Telegram Stars transferred by the transaction
    #[must_use]
    pub fn amount(&self) -> i64 {
        match self {
            Self::Incoming(val) => val.amount,
            Self::Outgoing(val) => val.amount,
        }
    }

    /// Helper method for field `date`.
    ///
    /// # Variants
    /// - `StarTransactionIncoming`. Date the transaction was created in Unix time
    /// - `StarTransactionOutgoing`. Date the transaction was created in Unix time
    #[must_use]
    pub fn date(&self) -> i64 {
        match self {
            Self::Incoming(val) => val.date,
            Self::Outgoing(val) => val.date,
        }
    }

    /// Helper method for field `id`.
    ///
    /// # Variants
    /// - `StarTransactionIncoming`. Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    /// - `StarTransactionOutgoing`. Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with [`SuccessfulPayment`].`telegram_payment_charge_id` for successful incoming payments from users.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Incoming(val) => val.id.as_ref(),
            Self::Outgoing(val) => val.id.as_ref(),
        }
    }

    /// Helper method for field `nanostar_amount`.
    ///
    /// # Variants
    /// - `StarTransactionIncoming`. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    /// - `StarTransactionOutgoing`. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[must_use]
    pub fn nanostar_amount(&self) -> Option<u32> {
        match self {
            Self::Incoming(val) => val.nanostar_amount,
            Self::Outgoing(val) => val.nanostar_amount,
        }
    }

    /// Helper method for field `receiver`.
    ///
    /// # Variants
    /// - `StarTransactionOutgoing`. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[must_use]
    pub fn receiver(&self) -> Option<&crate::types::TransactionPartner> {
        match self {
            Self::Outgoing(val) => Some(&val.receiver),
            Self::Incoming(_) => None,
        }
    }

    /// Helper method for field `source`.
    ///
    /// # Variants
    /// - `StarTransactionIncoming`. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[must_use]
    pub fn source(&self) -> Option<&crate::types::TransactionPartner> {
        match self {
            Self::Incoming(val) => Some(&val.source),
            Self::Outgoing(_) => None,
        }
    }

    /// Helper method for nested field `chat`.
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::chat(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::chat(inner)
            }
        }
    }

    /// Helper method for nested field `commission_per_mille`.
    #[must_use]
    pub fn commission_per_mille(&self) -> Option<i64> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::commission_per_mille(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::commission_per_mille(inner)
            }
        }
    }

    /// Helper method for nested field `gift`.
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::Gift> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::gift(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::gift(inner)
            }
        }
    }

    /// Helper method for nested field `request_count`.
    #[must_use]
    pub fn request_count(&self) -> Option<i64> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::request_count(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::request_count(inner)
            }
        }
    }

    /// Helper method for nested field `sponsor_user`.
    #[must_use]
    pub fn sponsor_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::sponsor_user(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::sponsor_user(inner)
            }
        }
    }

    /// Helper method for nested field `withdrawal_state`.
    #[must_use]
    pub fn withdrawal_state(&self) -> Option<&crate::types::RevenueWithdrawalState> {
        match self {
            Self::Outgoing(val) => {
                let inner = &val.receiver;
                crate::types::TransactionPartner::withdrawal_state(inner)
            }
            Self::Incoming(val) => {
                let inner = &val.source;
                crate::types::TransactionPartner::withdrawal_state(inner)
            }
        }
    }
}
impl From<crate::types::StarTransactionIncoming> for StarTransaction {
    fn from(val: crate::types::StarTransactionIncoming) -> Self {
        Self::Incoming(val)
    }
}
impl TryFrom<StarTransaction> for crate::types::StarTransactionIncoming {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StarTransaction) -> Result<Self, Self::Error> {
        match val {
            StarTransaction::Incoming(inner) => Ok(inner),
            StarTransaction::Outgoing(_) => Err(Self::Error::new(
                stringify!(StarTransaction),
                stringify!(StarTransactionIncoming),
            )),
        }
    }
}
impl From<crate::types::StarTransactionOutgoing> for StarTransaction {
    fn from(val: crate::types::StarTransactionOutgoing) -> Self {
        Self::Outgoing(val)
    }
}
impl TryFrom<StarTransaction> for crate::types::StarTransactionOutgoing {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StarTransaction) -> Result<Self, Self::Error> {
        match val {
            StarTransaction::Outgoing(inner) => Ok(inner),
            StarTransaction::Incoming(_) => Err(Self::Error::new(
                stringify!(StarTransaction),
                stringify!(StarTransactionOutgoing),
            )),
        }
    }
}
