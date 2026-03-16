use crate::types::StarTransaction;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
/// Currently, it can be one of
/// - [`crate::types::StarTransactionIncoming`]
/// - [`crate::types::StarTransactionOutgoing`]
/// # Documentation
/// <https://core.telegram.org/bots/api#startransaction>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum StarTransactionType {
    #[strum(serialize = "incoming")]
    Incoming,
    #[strum(serialize = "outgoing")]
    Outgoing,
}
impl StarTransactionType {
    #[must_use]
    pub const fn all() -> [StarTransactionType; 2usize] {
        [StarTransactionType::Incoming, StarTransactionType::Outgoing]
    }
}
impl From<StarTransactionType> for Box<str> {
    fn from(val: StarTransactionType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StarTransactionType> for String {
    fn from(val: StarTransactionType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StarTransactionType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a StarTransaction> for StarTransactionType {
    fn from(val: &'a StarTransaction) -> Self {
        match val {
            StarTransaction::Incoming(_) => StarTransactionType::Incoming,
            StarTransaction::Outgoing(_) => StarTransactionType::Outgoing,
        }
    }
}
impl From<StarTransaction> for StarTransactionType {
    fn from(val: StarTransaction) -> Self {
        StarTransactionType::from(&val)
    }
}
