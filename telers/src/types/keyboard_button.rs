use serde::{Deserialize, Serialize};
/// This object represents one button of the reply keyboard. At most one of the fields other than text, `icon_custom_emoji_id`, and style must be used to specify the type of the button. For simple text buttons, String can be used instead of this object to specify the button text.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the fields other than text, `icon_custom_emoji_id`, and style are used, it will be sent as a message when the button is pressed.
    pub text: Box<str>,
    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
    /// Style of the button. Must be one of `danger` (red), `success` (green) or `primary` (blue). If omitted, then an app-specific style is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<str>>,
    /// If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a `users_shared` service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<crate::types::KeyboardButtonRequestUsers>,
    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a `chat_shared` service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<crate::types::KeyboardButtonRequestChat>,
    /// If specified, pressing the button will ask the user to create and share a bot that will be managed by the current bot. Available for bots that enabled management of other bots in the @`BotFather` Mini App. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_managed_bot: Option<crate::types::KeyboardButtonRequestManagedBot>,
    /// If `true`, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// If `true`, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<crate::types::KeyboardButtonPollType>,
    /// If specified, the described Web App will be launched when the button is pressed. The Web App will be able to send a `web_app_data` service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::types::WebAppInfo>,
}
impl KeyboardButton {
    /// Creates a new `KeyboardButton`.
    ///
    /// # Arguments
    /// * `text` - Text of the button. If none of the fields other than text, `icon_custom_emoji_id`, and style are used, it will be sent as a message when the button is pressed.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(text: T0) -> Self {
        Self {
            text: text.into(),
            icon_custom_emoji_id: None,
            style: None,
            request_users: None,
            request_chat: None,
            request_managed_bot: None,
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }

    /// Text of the button. If none of the fields other than text, `icon_custom_emoji_id`, and style are used, it will be sent as a message when the button is pressed.
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[must_use]
    pub fn icon_custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[must_use]
    pub fn icon_custom_emoji_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.icon_custom_emoji_id = val.map(Into::into);
        self
    }

    /// Style of the button. Must be one of `danger` (red), `success` (green) or `primary` (blue). If omitted, then an app-specific style is used.
    #[must_use]
    pub fn style<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.style = Some(val.into());
        self
    }

    /// Style of the button. Must be one of `danger` (red), `success` (green) or `primary` (blue). If omitted, then an app-specific style is used.
    #[must_use]
    pub fn style_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.style = val.map(Into::into);
        self
    }

    /// If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a `users_shared` service message. Available in private chats only.
    #[must_use]
    pub fn request_users<T: Into<crate::types::KeyboardButtonRequestUsers>>(
        mut self,
        val: T,
    ) -> Self {
        self.request_users = Some(val.into());
        self
    }

    /// If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a `users_shared` service message. Available in private chats only.
    #[must_use]
    pub fn request_users_option<T: Into<crate::types::KeyboardButtonRequestUsers>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.request_users = val.map(Into::into);
        self
    }

    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a `chat_shared` service message. Available in private chats only.
    #[must_use]
    pub fn request_chat<T: Into<crate::types::KeyboardButtonRequestChat>>(
        mut self,
        val: T,
    ) -> Self {
        self.request_chat = Some(val.into());
        self
    }

    /// If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a `chat_shared` service message. Available in private chats only.
    #[must_use]
    pub fn request_chat_option<T: Into<crate::types::KeyboardButtonRequestChat>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.request_chat = val.map(Into::into);
        self
    }

    /// If specified, pressing the button will ask the user to create and share a bot that will be managed by the current bot. Available for bots that enabled management of other bots in the @`BotFather` Mini App. Available in private chats only.
    #[must_use]
    pub fn request_managed_bot<T: Into<crate::types::KeyboardButtonRequestManagedBot>>(
        mut self,
        val: T,
    ) -> Self {
        self.request_managed_bot = Some(val.into());
        self
    }

    /// If specified, pressing the button will ask the user to create and share a bot that will be managed by the current bot. Available for bots that enabled management of other bots in the @`BotFather` Mini App. Available in private chats only.
    #[must_use]
    pub fn request_managed_bot_option<T: Into<crate::types::KeyboardButtonRequestManagedBot>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.request_managed_bot = val.map(Into::into);
        self
    }

    /// If `true`, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_contact<T: Into<bool>>(mut self, val: T) -> Self {
        self.request_contact = Some(val.into());
        self
    }

    /// If `true`, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_contact_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.request_contact = val.map(Into::into);
        self
    }

    /// If `true`, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_location<T: Into<bool>>(mut self, val: T) -> Self {
        self.request_location = Some(val.into());
        self
    }

    /// If `true`, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_location_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.request_location = val.map(Into::into);
        self
    }

    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_poll<T: Into<crate::types::KeyboardButtonPollType>>(mut self, val: T) -> Self {
        self.request_poll = Some(val.into());
        self
    }

    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[must_use]
    pub fn request_poll_option<T: Into<crate::types::KeyboardButtonPollType>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.request_poll = val.map(Into::into);
        self
    }

    /// If specified, the described Web App will be launched when the button is pressed. The Web App will be able to send a `web_app_data` service message. Available in private chats only.
    #[must_use]
    pub fn web_app<T: Into<crate::types::WebAppInfo>>(mut self, val: T) -> Self {
        self.web_app = Some(val.into());
        self
    }

    /// If specified, the described Web App will be launched when the button is pressed. The Web App will be able to send a `web_app_data` service message. Available in private chats only.
    #[must_use]
    pub fn web_app_option<T: Into<crate::types::WebAppInfo>>(mut self, val: Option<T>) -> Self {
        self.web_app = val.map(Into::into);
        self
    }
}
