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
    /// Information about the regular gift
    pub gift: Box<crate::types::Gift>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl OwnedGiftUnknown {
    /// Creates a new `OwnedGiftUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `gift` - Information about the regular gift
    /// * `send_date` - Date the gift was sent in Unix time
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::Gift>, T2: Into<i64>>(
        r#type: T0,
        gift: T1,
        send_date: T2,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            gift: Box::new(gift.into()),
            send_date: send_date.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Information about the regular gift
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }

    /// Date the gift was sent in Unix time
    #[must_use]
    pub fn send_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.send_date = val.into();
        self
    }
}
