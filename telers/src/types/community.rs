use serde::{Deserialize, Serialize};
/// Represents a community (a group of chats).
/// # Documentation
/// <https://core.telegram.org/bots/api#community>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Community {
    /// Unique identifier for this community. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Name of the community
    pub name: Box<str>,
}
impl Community {
    /// Creates a new `Community`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this community. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `name` - Name of the community
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(id: T0, name: T1) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Unique identifier for this community. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Name of the community
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
