use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::ChatBoostSource`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostSourceUnknown {
    /// Raw `source` value of the variant unknown to this version of the library
    pub source: Box<str>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl ChatBoostSourceUnknown {
    /// Creates a new `ChatBoostSourceUnknown`.
    ///
    /// # Arguments
    /// * `source` - Raw `source` value of the variant unknown to this version of the library
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(source: T0) -> Self {
        Self {
            source: source.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `source` value of the variant unknown to this version of the library
    #[must_use]
    pub fn source<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.source = val.into();
        self
    }
}
