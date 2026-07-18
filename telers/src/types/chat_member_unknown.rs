use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::ChatMember`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberUnknown {
    /// Raw `status` value of the variant unknown to this version of the library
    pub status: Box<str>,
    /// Information about the user
    pub user: Box<crate::types::User>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl ChatMemberUnknown {
    /// Creates a new `ChatMemberUnknown`.
    ///
    /// # Arguments
    /// * `status` - Raw `status` value of the variant unknown to this version of the library
    /// * `user` - Information about the user
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::User>>(status: T0, user: T1) -> Self {
        Self {
            status: status.into(),
            user: Box::new(user.into()),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `status` value of the variant unknown to this version of the library
    #[must_use]
    pub fn status<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.status = val.into();
        self
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
