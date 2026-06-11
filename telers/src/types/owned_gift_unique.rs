use serde::{Deserialize, Serialize};
/// Describes a unique gift received and owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftunique>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGiftUnique {
    /// Information about the unique gift
    pub gift: Box<crate::types::UniqueGift>,
    /// Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<Box<str>>,
    /// Sender of the gift if it is a known user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<Box<crate::types::User>>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// `true`, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_transferred: Option<bool>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}
impl OwnedGiftUnique {
    /// Creates a new `OwnedGiftUnique`.
    ///
    /// # Arguments
    /// * `gift` - Information about the unique gift
    /// * `send_date` - Date the gift was sent in Unix time
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::UniqueGift>, T1: Into<i64>>(gift: T0, send_date: T1) -> Self {
        Self {
            gift: Box::new(gift.into()),
            owned_gift_id: None,
            sender_user: None,
            send_date: send_date.into(),
            is_saved: None,
            can_be_transferred: None,
            transfer_star_count: None,
            next_transfer_date: None,
        }
    }

    /// Information about the unique gift
    #[must_use]
    pub fn gift<T: Into<crate::types::UniqueGift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }

    /// Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = Some(val.into());
        self
    }

    /// Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.owned_gift_id = val.map(Into::into);
        self
    }

    /// Sender of the gift if it is a known user
    #[must_use]
    pub fn sender_user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.sender_user = Some(Box::new(val.into()));
        self
    }

    /// Sender of the gift if it is a known user
    #[must_use]
    pub fn sender_user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.sender_user = val.map(|val| Box::new(val.into()));
        self
    }

    /// Date the gift was sent in Unix time
    #[must_use]
    pub fn send_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.send_date = val.into();
        self
    }

    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_saved<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_saved = Some(val.into());
        self
    }

    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_saved_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_saved = val.map(Into::into);
        self
    }

    /// `true`, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_transferred<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_be_transferred = Some(val.into());
        self
    }

    /// `true`, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_transferred_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_be_transferred = val.map(Into::into);
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
