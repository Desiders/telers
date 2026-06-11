use serde::{Deserialize, Serialize};
/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberbanned>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever.
    pub until_date: i64,
}
impl ChatMemberBanned {
    /// Creates a new `ChatMemberBanned`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `until_date` - Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<i64>>(user: T0, until_date: T1) -> Self {
        Self {
            user: Box::new(user.into()),
            until_date: until_date.into(),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever.
    #[must_use]
    pub fn until_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.until_date = val.into();
        self
    }
}
