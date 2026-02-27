use serde::{Deserialize, Serialize};
/// Describes the connection of the bot with a business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessconnection>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: Box<str>,
    /// Business account user that created the business connection
    pub user: Box<crate::types::User>,
    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: i64,
    /// Date the connection was established in Unix time
    pub date: i64,
    /// Rights of the business bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<crate::types::BusinessBotRights>,
    /// `true`, if the connection is active
    pub is_enabled: bool,
}
impl BusinessConnection {
    /// Creates a new `BusinessConnection`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the business connection
    /// * `user` - Business account user that created the business connection
    /// * `user_chat_id` - Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `date` - Date the connection was established in Unix time
    /// * `is_enabled` - `true`, if the connection is active
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4: Into<bool>,
    >(
        id: T0,
        user: T1,
        user_chat_id: T2,
        date: T3,
        is_enabled: T4,
    ) -> Self {
        Self {
            id: id.into(),
            user: Box::new(user.into()),
            user_chat_id: user_chat_id.into(),
            date: date.into(),
            rights: None,
            is_enabled: is_enabled.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Business account user that created the business connection
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn user_chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_chat_id = val.into();
        this
    }

    /// Date the connection was established in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Rights of the business bot
    #[must_use]
    pub fn rights<T: Into<crate::types::BusinessBotRights>>(self, val: T) -> Self {
        let mut this = self;
        this.rights = Some(val.into());
        this
    }

    /// Rights of the business bot
    #[must_use]
    pub fn rights_option<T: Into<crate::types::BusinessBotRights>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.rights = val.map(Into::into);
        this
    }

    /// `true`, if the connection is active
    #[must_use]
    pub fn is_enabled<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_enabled = val.into();
        this
    }
}
