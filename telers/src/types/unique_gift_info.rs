use serde::{Deserialize, Serialize};
/// Describes a service message about a unique gift that was sent or received.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: Box<crate::types::UniqueGift>,
    /// Origin of the gift. Currently, either `upgrade` for gifts upgraded from regular gifts, `transfer` for gifts transferred from other users or channels, `resale` for gifts bought from other users, `gifted_upgrade` for upgrades purchased after the gift was sent, or `offer` for gifts bought or sold through gift purchase offers.
    pub origin: Box<str>,
    /// Text of the message that was added to the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Special entities that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of `XTR` for Telegram Stars or `TON` for TON grams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_currency: Option<Box<str>>,
    /// For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanograms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_amount: Option<i64>,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<Box<str>>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}
impl UniqueGiftInfo {
    /// Creates a new `UniqueGiftInfo`.
    ///
    /// # Arguments
    /// * `gift` - Information about the gift
    /// * `origin` - Origin of the gift. Currently, either `upgrade` for gifts upgraded from regular gifts, `transfer` for gifts transferred from other users or channels, `resale` for gifts bought from other users, `gifted_upgrade` for upgrades purchased after the gift was sent, or `offer` for gifts bought or sold through gift purchase offers.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::UniqueGift>, T1: Into<Box<str>>>(
        gift: T0,
        origin: T1,
    ) -> Self {
        Self {
            gift: Box::new(gift.into()),
            origin: origin.into(),
            text: None,
            entities: None,
            is_private: None,
            last_resale_currency: None,
            last_resale_amount: None,
            owned_gift_id: None,
            transfer_star_count: None,
            next_transfer_date: None,
        }
    }

    /// Information about the gift
    #[must_use]
    pub fn gift<T: Into<crate::types::UniqueGift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }

    /// Origin of the gift. Currently, either `upgrade` for gifts upgraded from regular gifts, `transfer` for gifts transferred from other users or channels, `resale` for gifts bought from other users, `gifted_upgrade` for upgrades purchased after the gift was sent, or `offer` for gifts bought or sold through gift purchase offers.
    #[must_use]
    pub fn origin<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.origin = val.into();
        self
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = Some(val.into());
        self
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.text = val.map(Into::into);
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.entities = val.map(Into::into);
        self
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_private = Some(val.into());
        self
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_private = val.map(Into::into);
        self
    }

    /// For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of `XTR` for Telegram Stars or `TON` for TON grams.
    #[must_use]
    pub fn last_resale_currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.last_resale_currency = Some(val.into());
        self
    }

    /// For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of `XTR` for Telegram Stars or `TON` for TON grams.
    #[must_use]
    pub fn last_resale_currency_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.last_resale_currency = val.map(Into::into);
        self
    }

    /// For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanograms
    #[must_use]
    pub fn last_resale_amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.last_resale_amount = Some(val.into());
        self
    }

    /// For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanograms
    #[must_use]
    pub fn last_resale_amount_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.last_resale_amount = val.map(Into::into);
        self
    }

    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = Some(val.into());
        self
    }

    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[must_use]
    pub fn owned_gift_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.owned_gift_id = val.map(Into::into);
        self
    }

    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[must_use]
    pub fn transfer_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.transfer_star_count = Some(val.into());
        self
    }

    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[must_use]
    pub fn transfer_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.transfer_star_count = val.map(Into::into);
        self
    }

    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[must_use]
    pub fn next_transfer_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.next_transfer_date = Some(val.into());
        self
    }

    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[must_use]
    pub fn next_transfer_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.next_transfer_date = val.map(Into::into);
        self
    }
}
