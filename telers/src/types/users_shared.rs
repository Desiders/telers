use serde::{Deserialize, Serialize};
/// This object contains information about the users whose identifiers were shared with the bot using a [`KeyboardButtonRequestUsers`] button.
/// # Documentation
/// <https://core.telegram.org/bots/api#usersshared>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Information about users shared with the bot.
    pub users: Box<[crate::types::SharedUser]>,
}
impl UsersShared {
    /// Creates a new `UsersShared`.
    ///
    /// # Arguments
    /// * `request_id` - Identifier of the request
    /// * `users` - Information about users shared with the bot.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<crate::types::SharedUser>,
        T1: IntoIterator<Item = T1Item>,
    >(
        request_id: T0,
        users: T1,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            users: users.into_iter().map(Into::into).collect(),
        }
    }

    /// Identifier of the request
    #[must_use]
    pub fn request_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.request_id = val.into();
        this
    }

    /// Information about users shared with the bot.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn users<T: Into<Box<[crate::types::SharedUser]>>>(self, val: T) -> Self {
        let mut this = self;
        this.users = this
            .users
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Information about users shared with the bot.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn user<T: Into<crate::types::SharedUser>>(self, val: T) -> Self {
        let mut this = self;
        this.users = this
            .users
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
