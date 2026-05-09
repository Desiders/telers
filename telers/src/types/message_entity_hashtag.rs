use serde::{Deserialize, Serialize};
/// This object represents a/an hashtag message entity.
/// # Notes
/// This object represents a message entity from original field `hashtag`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityHashtag {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
}
impl MessageEntityHashtag {
    /// Creates a new `MessageEntityHashtag`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(offset: T0, length: T1) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
        }
    }

    /// Offset in UTF-16 code units to the start of the entity
    #[must_use]
    pub fn offset<T: Into<i64>>(mut self, val: T) -> Self {
        self.offset = val.into();
        self
    }

    /// Length of the entity in UTF-16 code units
    #[must_use]
    pub fn length<T: Into<i64>>(mut self, val: T) -> Self {
        self.length = val.into();
        self
    }
}
