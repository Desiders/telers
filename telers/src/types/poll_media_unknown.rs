use std::collections::BTreeMap;
/// This object represents a [`crate::types::PollMedia`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct PollMediaUnknown {
    #[cfg_attr(not(feature = "deser"), serde(flatten))]
    #[cfg_attr(feature = "deser", deser(flatten))]
    pub extra: BTreeMap<Box<str>, crate::serialization::Value>,
}
impl PollMediaUnknown {
    /// Creates a new `PollMediaUnknown`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            extra: BTreeMap::new(),
        }
    }
}
impl Default for PollMediaUnknown {
    fn default() -> Self {
        Self::new()
    }
}
