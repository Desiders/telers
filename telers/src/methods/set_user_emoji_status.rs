use crate::client::Bot;
use serde::Serialize;
/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method request[`EmojiStatusAccess`]. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setuseremojistatus>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetUserEmojiStatus {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}
impl SetUserEmojiStatus {
    /// Creates a new `SetUserEmojiStatus`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
        }
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[must_use]
    pub fn emoji_status_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_status_custom_emoji_id = Some(val.into());
        this
    }

    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[must_use]
    pub fn emoji_status_custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.emoji_status_custom_emoji_id = val.map(Into::into);
        this
    }

    /// Expiration date of the emoji status, if any
    #[must_use]
    pub fn emoji_status_expiration_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_status_expiration_date = Some(val.into());
        this
    }

    /// Expiration date of the emoji status, if any
    #[must_use]
    pub fn emoji_status_expiration_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.emoji_status_expiration_date = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetUserEmojiStatus {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setUserEmojiStatus", self, None)
    }
}
