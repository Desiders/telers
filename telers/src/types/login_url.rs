use serde::{Deserialize, Serialize};
/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
/// Telegram apps support these buttons as of version 5.7.
/// # Documentation
/// <https://core.telegram.org/bots/api#loginurl>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data. NOTE: You must always check the hash of the received data to verify the authentication and the integrity of the data as described in Checking authorization.
    pub url: Box<str>,
    /// New text of the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<Box<str>>,
    /// Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<Box<str>>,
    /// Pass `true` to request the permission for your bot to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}
impl LoginUrl {
    /// Creates a new `LoginUrl`.
    ///
    /// # Arguments
    /// * `url` - An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data. NOTE: You must always check the hash of the received data to verify the authentication and the integrity of the data as described in Checking authorization.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data. NOTE: You must always check the hash of the received data to verify the authentication and the integrity of the data as described in Checking authorization.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.url = val.into();
        this
    }

    /// New text of the button in forwarded messages.
    #[must_use]
    pub fn forward_text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.forward_text = Some(val.into());
        this
    }

    /// New text of the button in forwarded messages.
    #[must_use]
    pub fn forward_text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.forward_text = val.map(Into::into);
        this
    }

    /// Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[must_use]
    pub fn bot_username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.bot_username = Some(val.into());
        this
    }

    /// Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[must_use]
    pub fn bot_username_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.bot_username = val.map(Into::into);
        this
    }

    /// Pass `true` to request the permission for your bot to send messages to the user.
    #[must_use]
    pub fn request_write_access<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.request_write_access = Some(val.into());
        this
    }

    /// Pass `true` to request the permission for your bot to send messages to the user.
    #[must_use]
    pub fn request_write_access_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.request_write_access = val.map(Into::into);
        this
    }
}
