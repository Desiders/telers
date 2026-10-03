use std::collections::BTreeMap;
/// This object represents a [`crate::types::MessageOrigin`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageorigin>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct MessageOriginUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Date the message was sent originally in Unix time
    pub date: i64,
    #[cfg_attr(not(feature = "deser"), serde(flatten))]
    #[cfg_attr(feature = "deser", deser(flatten))]
    pub extra: BTreeMap<Box<str>, crate::serialization::Value>,
}
impl MessageOriginUnknown {
    /// Creates a new `MessageOriginUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `date` - Date the message was sent originally in Unix time
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(r#type: T0, date: T1) -> Self {
        Self {
            r#type: r#type.into(),
            date: date.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }
}
