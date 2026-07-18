use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::MaybeInaccessibleMessage`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaybeInaccessibleMessageUnknown {
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl MaybeInaccessibleMessageUnknown {
    /// Creates a new `MaybeInaccessibleMessageUnknown`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            extra: BTreeMap::new(),
        }
    }
}
impl Default for MaybeInaccessibleMessageUnknown {
    fn default() -> Self {
        Self::new()
    }
}
