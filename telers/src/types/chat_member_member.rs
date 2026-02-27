use serde::{Deserialize, Serialize};
/// Represents a chat member that has no additional privileges or restrictions.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberMember {
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
            user: Box::new(user.into()),
            until_date: None,
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// Date when the user's subscription will expire; Unix time
    #[must_use]
    pub fn until_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.until_date = Some(val.into());
        this
    }

    /// Date when the user's subscription will expire; Unix time
    #[must_use]
    pub fn until_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.until_date = val.map(Into::into);
        this
    }
}
