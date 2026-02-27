use serde::{Deserialize, Serialize};
/// Contains information about the affiliate that received a commission via this transaction.
/// # Documentation
/// <https://core.telegram.org/bots/api#affiliateinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AffiliateInfo {
    /// The bot or the user that received an affiliate commission if it was received by a bot or a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_user: Option<Box<crate::types::User>>,
    /// The chat that received an affiliate commission if it was received by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_chat: Option<Box<crate::types::Chat>>,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    pub commission_per_mille: i64,
    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i32>,
}
impl AffiliateInfo {
    /// Creates a new `AffiliateInfo`.
    ///
    /// # Arguments
    /// * `commission_per_mille` - The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    /// * `amount` - Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(commission_per_mille: T0, amount: T1) -> Self {
        Self {
            affiliate_user: None,
            affiliate_chat: None,
            commission_per_mille: commission_per_mille.into(),
            amount: amount.into(),
            nanostar_amount: None,
        }
    }

    /// The bot or the user that received an affiliate commission if it was received by a bot or a user
    #[must_use]
    pub fn affiliate_user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.affiliate_user = Some(Box::new(val.into()));
        this
    }

    /// The bot or the user that received an affiliate commission if it was received by a bot or a user
    #[must_use]
    pub fn affiliate_user_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.affiliate_user = val.map(|val| Box::new(val.into()));
        this
    }

    /// The chat that received an affiliate commission if it was received by a chat
    #[must_use]
    pub fn affiliate_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.affiliate_chat = Some(Box::new(val.into()));
        this
    }

    /// The chat that received an affiliate commission if it was received by a chat
    #[must_use]
    pub fn affiliate_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.affiliate_chat = val.map(|val| Box::new(val.into()));
        this
    }

    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    #[must_use]
    pub fn commission_per_mille<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.commission_per_mille = val.into();
        this
    }

    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    #[must_use]
    pub fn amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.amount = val.into();
        this
    }

    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    #[must_use]
    pub fn nanostar_amount<T: Into<i32>>(self, val: T) -> Self {
        let mut this = self;
        this.nanostar_amount = Some(val.into());
        this
    }

    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    #[must_use]
    pub fn nanostar_amount_option<T: Into<i32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.nanostar_amount = val.map(Into::into);
        this
    }
}
