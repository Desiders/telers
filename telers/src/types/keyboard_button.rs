use super::{
    KeyboardButtonPollType, KeyboardButtonRequestChat, KeyboardButtonRequestUsers, WebAppInfo,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents one button of the reply keyboard. For simple text buttons *String* can be used instead of this object to specify text of the button. Optional fields `web_app`, `request_users`, `request_chat`, `request_contact`, `request_location`, and `request_poll` are mutually exclusive.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbutton>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a `users_shared` service message. Available in private chats only.
    pub request_users: Option<KeyboardButtonRequestUsers>,
    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a `chat_shared` service message. Available in private chats only.
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// If `true`, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    pub request_contact: Option<bool>,
    /// If `true`, the user's current location will be sent when the button is pressed. Available in private chats only.
    pub request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    pub request_poll: Option<KeyboardButtonPollType>,
    /// If specified, the described [`Web App`](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a 'web_app_data' service message. Available in private chats only.
    pub web_app: Option<WebAppInfo>,
}

impl KeyboardButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            request_users: None,
            request_chat: None,
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn request_user(self, val: KeyboardButtonRequestUsers) -> Self {
        Self {
            request_users: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn request_chat(self, val: KeyboardButtonRequestChat) -> Self {
        Self {
            request_chat: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn request_contact(self, val: bool) -> Self {
        Self {
            request_contact: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn request_location(self, val: bool) -> Self {
        Self {
            request_location: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn request_poll(self, val: KeyboardButtonPollType) -> Self {
        Self {
            request_poll: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn web_app(self, val: WebAppInfo) -> Self {
        Self {
            web_app: Some(val),
            ..self
        }
    }
}

impl KeyboardButton {
    #[must_use]
    pub fn request_user_option(self, val: Option<KeyboardButtonRequestUsers>) -> Self {
        Self {
            request_users: val,
            ..self
        }
    }

    #[must_use]
    pub fn request_chat_option(self, val: Option<KeyboardButtonRequestChat>) -> Self {
        Self {
            request_chat: val,
            ..self
        }
    }

    #[must_use]
    pub fn request_contact_option(self, val: Option<bool>) -> Self {
        Self {
            request_contact: val,
            ..self
        }
    }

    #[must_use]
    pub fn request_location_option(self, val: Option<bool>) -> Self {
        Self {
            request_location: val,
            ..self
        }
    }

    #[must_use]
    pub fn request_poll_option(self, val: Option<KeyboardButtonPollType>) -> Self {
        Self {
            request_poll: val,
            ..self
        }
    }

    #[must_use]
    pub fn web_app_option(self, val: Option<WebAppInfo>) -> Self {
        Self {
            web_app: val,
            ..self
        }
    }
}
