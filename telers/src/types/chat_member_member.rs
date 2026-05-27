use serde::{Deserialize, Serialize};
/// Represents a chat member that has no additional privileges or restrictions.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// Tag of the member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Box<str>>,
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// Date when the user's subscription will expire; Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
impl ChatMemberMember {
    /// Creates a new `ChatMemberMember`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            tag: None,
            user: Box::new(user.into()),
            until_date: None,
        }
    }

    /// Tag of the member
    #[must_use]
    pub fn tag<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.tag = Some(val.into());
        self
    }

    /// Tag of the member
    #[must_use]
    pub fn tag_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.tag = val.map(Into::into);
        self
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// Date when the user's subscription will expire; Unix time
    #[must_use]
    pub fn until_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.until_date = Some(val.into());
        self
    }

    /// Date when the user's subscription will expire; Unix time
    #[must_use]
    pub fn until_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.until_date = val.map(Into::into);
        self
    }
}
