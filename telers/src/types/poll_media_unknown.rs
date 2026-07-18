use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::PollMedia`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaUnknown {
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
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
