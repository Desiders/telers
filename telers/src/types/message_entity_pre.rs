use serde::{Deserialize, Serialize};
/// This object represents a/an pre message entity.
/// # Notes
/// This object represents a message entity from original field `pre`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityPre {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For `pre` only, the programming language of the entity text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<str>>,
}
impl MessageEntityPre {
    /// Creates a new `MessageEntityPre`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(offset: T0, length: T1) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
            language: None,
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

    /// For `pre` only, the programming language of the entity text
    #[must_use]
    pub fn language<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language = Some(val.into());
        self
    }

    /// For `pre` only, the programming language of the entity text
    #[must_use]
    pub fn language_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language = val.map(Into::into);
        self
    }
}
