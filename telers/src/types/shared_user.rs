use serde::{Deserialize, Serialize};
/// This object contains information about a user that was shared with the bot using a [`crate::types::KeyboardButtonRequestUsers`] button.
/// # Documentation
/// <https://core.telegram.org/bots/api#shareduser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SharedUser {
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: i64,
    /// First name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<str>>,
    /// Last name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// Username of the user, if the username was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<[crate::types::PhotoSize]>>,
}
impl SharedUser {
    /// Creates a new `SharedUser`.
    ///
    /// # Arguments
    /// * `user_id` - Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            first_name: None,
            last_name: None,
            username: None,
            photo: None,
        }
    }

    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// First name of the user, if the name was requested by the bot
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.first_name = Some(val.into());
        this
    }

    /// First name of the user, if the name was requested by the bot
    #[must_use]
    pub fn first_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.first_name = val.map(Into::into);
        this
    }

    /// Last name of the user, if the name was requested by the bot
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.last_name = Some(val.into());
        this
    }

    /// Last name of the user, if the name was requested by the bot
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.last_name = val.map(Into::into);
        this
    }

    /// Username of the user, if the username was requested by the bot
    #[must_use]
    pub fn username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.username = Some(val.into());
        this
    }

    /// Username of the user, if the username was requested by the bot
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.username = val.map(Into::into);
        this
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = Some(
            this.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = Some(
            this.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo_option<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo = val.map(Into::into);
        this
    }
}
