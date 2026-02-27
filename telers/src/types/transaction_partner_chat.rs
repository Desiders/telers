use serde::{Deserialize, Serialize};
/// Describes a transaction with a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerchat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerChat {
    /// Information about the chat
    pub chat: Box<crate::types::Chat>,
    /// The gift sent to the chat by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<crate::types::Gift>>,
}
impl TransactionPartnerChat {
    /// Creates a new `TransactionPartnerChat`.
    ///
    /// # Arguments
    /// * `chat` - Information about the chat
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Chat>>(chat: T0) -> Self {
        Self {
            chat: Box::new(chat.into()),
            gift: None,
        }
    }

    /// Information about the chat
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// The gift sent to the chat by the bot
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(self, val: T) -> Self {
        let mut this = self;
        this.gift = Some(Box::new(val.into()));
        this
    }

    /// The gift sent to the chat by the bot
    #[must_use]
    pub fn gift_option<T: Into<crate::types::Gift>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.gift = val.map(|val| Box::new(val.into()));
        this
    }
}
