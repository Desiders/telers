use serde::{Deserialize, Serialize};
/// This object represents a/an text mention message entity.
/// # Notes
/// This object represents a message entity from original field `text_mention`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityTextMention {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For `text_mention` only, the mentioned user
    pub user: Box<crate::types::User>,
}
impl MessageEntityTextMention {
    /// Creates a new `MessageEntityTextMention`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    /// * `user` - For `text_mention` only, the mentioned user
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<crate::types::User>>(
        offset: T0,
        length: T1,
        user: T2,
    ) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
            user: Box::new(user.into()),
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

    /// For `text_mention` only, the mentioned user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
