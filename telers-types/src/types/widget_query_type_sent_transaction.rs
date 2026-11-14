/// The query notifies the bot about a transaction sent from a TON wallet of the user who sent the query.
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerytypesenttransaction>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetQueryTypeSentTransaction {
    /// The wallet from which the transaction was sent
    pub wallet: crate::types::WidgetQueryWallet,
    /// Value of `msg_hash` of the transaction
    pub msg_hash: Box<str>,
}
impl WidgetQueryTypeSentTransaction {
    /// Creates a new `WidgetQueryTypeSentTransaction`.
    ///
    /// # Arguments
    /// * `wallet` - The wallet from which the transaction was sent
    /// * `msg_hash` - Value of `msg_hash` of the transaction
    #[must_use]
    pub fn new<T0: Into<crate::types::WidgetQueryWallet>, T1: Into<Box<str>>>(
        wallet: T0,
        msg_hash: T1,
    ) -> Self {
        Self {
            wallet: wallet.into(),
            msg_hash: msg_hash.into(),
        }
    }

    /// The wallet from which the transaction was sent
    #[must_use]
    pub fn wallet<T: Into<crate::types::WidgetQueryWallet>>(mut self, val: T) -> Self {
        self.wallet = val.into();
        self
    }

    /// Value of `msg_hash` of the transaction
    #[must_use]
    pub fn msg_hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.msg_hash = val.into();
        self
    }
}
