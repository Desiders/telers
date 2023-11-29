use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the [`Telegram Login Widget`](https://core.telegram.org/widgets/login) when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
/// Telegram apps support these buttons as of [`version 5.7`](https://telegram.org/blog/privacy-discussions-web-bots#meet-seamless-web-bots).
/// Sample bot: [`@discussbot`](https://t.me/discussbot)
/// # Documentation
/// <https://core.telegram.org/bots/api#loginurl>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in [`Receiving authorization data`](https://core.telegram.org/widgets/login#receiving-authorization-data).
    pub url: String,
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization. See [`Setting up a bot`](https://core.telegram.org/widgets/login#setting-up-a-bot) for more details. If not specified, the current bot's username will be assumed. The *url*'s domain must be the same as the domain linked with the bot. See [`Linking your domain to the bot`](https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot) for more details.
    pub bot_username: Option<String>,
    /// Pass `true` to request the permission for your bot to send messages to the user.
    pub request_write_access: Option<bool>,
}

impl LoginUrl {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    #[must_use]
    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn forward_text(self, val: impl Into<String>) -> Self {
        Self {
            forward_text: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn bot_username(self, val: impl Into<String>) -> Self {
        Self {
            bot_username: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn request_write_access(self, val: bool) -> Self {
        Self {
            request_write_access: Some(val),
            ..self
        }
    }
}

impl LoginUrl {
    #[must_use]
    pub fn forward_text_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            forward_text: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn bot_username_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            bot_username: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn request_write_access_option(self, val: Option<bool>) -> Self {
        Self {
            request_write_access: val,
            ..self
        }
    }
}
