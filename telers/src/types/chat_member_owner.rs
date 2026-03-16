use serde::{Deserialize, Serialize};
/// Represents a chat member that owns the chat and has all administrator privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<Box<str>>,
}
impl ChatMemberOwner {
    /// Creates a new `ChatMemberOwner`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `is_anonymous` - `true`, if the user's presence in the chat is hidden
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<bool>>(user: T0, is_anonymous: T1) -> Self {
        Self {
            user: Box::new(user.into()),
            is_anonymous: is_anonymous.into(),
            custom_title: None,
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// `true`, if the user's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_anonymous = val.into();
        this
    }

    /// Custom title for this user
    #[must_use]
    pub fn custom_title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.custom_title = Some(val.into());
        this
    }

    /// Custom title for this user
    #[must_use]
    pub fn custom_title_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.custom_title = val.map(Into::into);
        this
    }
}
