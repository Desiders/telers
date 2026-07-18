use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::OwnedGift`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgift>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGiftUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
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
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl OwnedGiftUnknown {
    /// Creates a new `OwnedGiftUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `send_date` - Date the gift was sent in Unix time
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(r#type: T0, send_date: T1) -> Self {
        Self {
            r#type: r#type.into(),
            owned_gift_id: None,
            sender_user: None,
            send_date: send_date.into(),
            is_saved: None,
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = Some(val.into());
        self
    }

    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
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
}
