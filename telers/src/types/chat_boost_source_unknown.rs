use std::collections::BTreeMap;
/// This object represents a [`crate::types::ChatBoostSource`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatBoostSourceUnknown {
    /// Raw `source` value of the variant unknown to this version of the library
    pub source: Box<str>,
    /// User that boosted the chat
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub user: Option<Box<crate::types::User>>,
    #[cfg_attr(not(feature = "deser"), serde(flatten))]
    #[cfg_attr(feature = "deser", deser(flatten))]
    pub extra: BTreeMap<Box<str>, crate::serialization::Value>,
}
impl ChatBoostSourceUnknown {
    /// Creates a new `ChatBoostSourceUnknown`.
    ///
    /// # Arguments
    /// * `source` - Raw `source` value of the variant unknown to this version of the library
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(source: T0) -> Self {
        Self {
            source: source.into(),
            user: None,
            extra: BTreeMap::new(),
        }
    }

    /// Raw `source` value of the variant unknown to this version of the library
    #[must_use]
    pub fn source<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.source = val.into();
        self
    }

    /// User that boosted the chat
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Some(Box::new(val.into()));
        self
    }

    /// User that boosted the chat
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.user = val.map(|val| Box::new(val.into()));
        self
    }
}
