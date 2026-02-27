use serde::{Deserialize, Serialize};
/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. More about requesting users: <https://core.telegram.org/bots/features#chat-and-user-selection>
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestusers>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the [`UsersShared`] object. Must be unique within the message
    pub request_id: i64,
    /// Pass `true` to request bots, pass `false` to request regular users. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// Pass `true` to request premium users, pass `false` to request non-premium users. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<u8>,
    /// Pass `true` to request the users' first and last names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// Pass `true` to request the users' usernames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Pass `true` to request the users' photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}
impl KeyboardButtonRequestUsers {
    /// Creates a new `KeyboardButtonRequestUsers`.
    ///
    /// # Arguments
    /// * `request_id` - Signed 32-bit identifier of the request that will be received back in the [`UsersShared`] object. Must be unique within the message
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(request_id: T0) -> Self {
        Self {
            request_id: request_id.into(),
            user_is_bot: None,
            user_is_premium: None,
            max_quantity: None,
            request_name: None,
            request_username: None,
            request_photo: None,
        }
    }

    /// Signed 32-bit identifier of the request that will be received back in the [`UsersShared`] object. Must be unique within the message
    #[must_use]
    pub fn request_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.request_id = val.into();
        this
    }

    /// Pass `true` to request bots, pass `false` to request regular users. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_is_bot<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.user_is_bot = Some(val.into());
        this
    }

    /// Pass `true` to request bots, pass `false` to request regular users. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_is_bot_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user_is_bot = val.map(Into::into);
        this
    }

    /// Pass `true` to request premium users, pass `false` to request non-premium users. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_is_premium<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.user_is_premium = Some(val.into());
        this
    }

    /// Pass `true` to request premium users, pass `false` to request non-premium users. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_is_premium_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user_is_premium = val.map(Into::into);
        this
    }

    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    #[must_use]
    pub fn max_quantity<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.max_quantity = Some(val.into());
        this
    }

    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    #[must_use]
    pub fn max_quantity_option<T: Into<u8>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.max_quantity = val.map(Into::into);
        this
    }

    /// Pass `true` to request the users' first and last names
    #[must_use]
    pub fn request_name<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.request_name = Some(val.into());
        this
    }

    /// Pass `true` to request the users' first and last names
    #[must_use]
    pub fn request_name_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.request_name = val.map(Into::into);
        this
    }

    /// Pass `true` to request the users' usernames
    #[must_use]
    pub fn request_username<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.request_username = Some(val.into());
        this
    }

    /// Pass `true` to request the users' usernames
    #[must_use]
    pub fn request_username_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.request_username = val.map(Into::into);
        this
    }

    /// Pass `true` to request the users' photos
    #[must_use]
    pub fn request_photo<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.request_photo = Some(val.into());
        this
    }

    /// Pass `true` to request the users' photos
    #[must_use]
    pub fn request_photo_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.request_photo = val.map(Into::into);
        this
    }
}
