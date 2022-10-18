use super::{KeyboardButtonPollType, WebAppInfo};

use serde::{Deserialize, Serialize};

/// This object represents one button of the reply keyboard. For simple text buttons *String* can be used instead of this object to specify text of the button. Optional fields `web_app`, `request_contact`, `request_location`, and `request_poll` are mutually exclusive.
/// **Note:** `request_contact` and `request_location` options will only work in Telegram versions released after 9 April, 2016. Older clients will display *unsupported message*.
/// **Note:** `request_poll` option will only work in Telegram versions released after 23 January, 2020. Older clients will display *unsupported message*.
/// **Note:** `web_app` option will only work in Telegram versions released after 16 April, 2022. Older clients will display *unsupported message*.
/// <https://core.telegram.org/bots/api#keyboardbutton>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// *Optional*. If :code:`True`, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    pub request_contact: Option<bool>,
    /// *Optional*. If :code:`True`, the user's current location will be sent when the button is pressed. Available in private chats only.
    pub request_location: Option<bool>,
    /// *Optional*. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    pub request_poll: Option<KeyboardButtonPollType>,
    /// *Optional*. If specified, the described `Web App <https://core.telegram.org/bots/webapps>`_ will be launched when the button is pressed. The Web App will be able to send a 'web_app_data' service message. Available in private chats only.
    pub web_app: Option<WebAppInfo>,
}

impl Default for KeyboardButton {
    fn default() -> Self {
        Self {
            text: String::default(),
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }
}
