use std::collections::BTreeMap;
/// This object represents a [`crate::types::PaidMedia`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedia>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct PaidMediaUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    #[cfg_attr(not(feature = "deser"), serde(flatten))]
    #[cfg_attr(feature = "deser", deser(flatten))]
    pub extra: BTreeMap<Box<str>, crate::serialization::Value>,
}
impl PaidMediaUnknown {
    /// Creates a new `PaidMediaUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(r#type: T0) -> Self {
        Self {
            r#type: r#type.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }
}
