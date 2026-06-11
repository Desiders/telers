use serde::{Deserialize, Serialize};
/// This object represents one button of an inline keyboard. Exactly one of the fields other than text, `icon_custom_emoji_id`, and style must be used to specify the type of the button.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: Box<str>,
    /// Unique identifier of the custom emoji shown before the text of the button. Can only be used by bots that purchased additional usernames on Fragment or in the messages directly sent by the bot to private, group and supergroup chats if the owner of the bot has a Telegram Premium subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
    /// Style of the button. Must be one of `danger` (red), `success` (green) or `primary` (blue). If omitted, then an app-specific style is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<str>>,
    /// HTTP or tg:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<Box<str>>,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::types::WebAppInfo>,
    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<crate::types::LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<Box<str>>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<Box<str>>,
    /// If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<crate::types::SwitchInlineQueryChosenChat>,
    /// Description of the button that copies the specified text to the clipboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<crate::types::CopyTextButton>,
    /// Description of the game that will be launched when the user presses the button. NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<crate::types::CallbackGame>,
    /// Specify `true`, to send a Pay button. Substrings `⭐` and `XTR` in the buttons's text will be replaced with a Telegram Star icon. NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}
impl InlineKeyboardButton {
    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    /// * `text` - Label text on the button
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(text: T0) -> Self {
        Self {
            text: text.into(),
            icon_custom_emoji_id: None,
            style: None,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            callback_game: None,
            pay: None,
        }
    }

    /// Label text on the button
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

    /// HTTP or tg:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    /// HTTP or tg:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[must_use]
    pub fn url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.url = val.map(Into::into);
        self
    }

    /// Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes
    #[must_use]
    pub fn callback_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_data = Some(val.into());
        self
    }

    /// Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes
    #[must_use]
    pub fn callback_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_data = val.map(Into::into);
        self
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[must_use]
    pub fn web_app<T: Into<crate::types::WebAppInfo>>(mut self, val: T) -> Self {
        self.web_app = Some(val.into());
        self
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[must_use]
    pub fn web_app_option<T: Into<crate::types::WebAppInfo>>(mut self, val: Option<T>) -> Self {
        self.web_app = val.map(Into::into);
        self
    }

    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[must_use]
    pub fn login_url<T: Into<crate::types::LoginUrl>>(mut self, val: T) -> Self {
        self.login_url = Some(val.into());
        self
    }

    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[must_use]
    pub fn login_url_option<T: Into<crate::types::LoginUrl>>(mut self, val: Option<T>) -> Self {
        self.login_url = val.map(Into::into);
        self
    }

    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.switch_inline_query = Some(val.into());
        self
    }

    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.switch_inline_query = val.map(Into::into);
        self
    }

    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_current_chat<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.switch_inline_query_current_chat = Some(val.into());
        self
    }

    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_current_chat_option<T: Into<Box<str>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.switch_inline_query_current_chat = val.map(Into::into);
        self
    }

    /// If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_chosen_chat<T: Into<crate::types::SwitchInlineQueryChosenChat>>(
        mut self,
        val: T,
    ) -> Self {
        self.switch_inline_query_chosen_chat = Some(val.into());
        self
    }

    /// If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_chosen_chat_option<
        T: Into<crate::types::SwitchInlineQueryChosenChat>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.switch_inline_query_chosen_chat = val.map(Into::into);
        self
    }

    /// Description of the button that copies the specified text to the clipboard
    #[must_use]
    pub fn copy_text<T: Into<crate::types::CopyTextButton>>(mut self, val: T) -> Self {
        self.copy_text = Some(val.into());
        self
    }

    /// Description of the button that copies the specified text to the clipboard
    #[must_use]
    pub fn copy_text_option<T: Into<crate::types::CopyTextButton>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.copy_text = val.map(Into::into);
        self
    }

    /// Description of the game that will be launched when the user presses the button. NOTE: This type of button must always be the first button in the first row.
    #[must_use]
    pub fn callback_game<T: Into<crate::types::CallbackGame>>(mut self, val: T) -> Self {
        self.callback_game = Some(val.into());
        self
    }

    /// Description of the game that will be launched when the user presses the button. NOTE: This type of button must always be the first button in the first row.
    #[must_use]
    pub fn callback_game_option<T: Into<crate::types::CallbackGame>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.callback_game = val.map(Into::into);
        self
    }

    /// Specify `true`, to send a Pay button. Substrings `⭐` and `XTR` in the buttons's text will be replaced with a Telegram Star icon. NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[must_use]
    pub fn pay<T: Into<bool>>(mut self, val: T) -> Self {
        self.pay = Some(val.into());
        self
    }

    /// Specify `true`, to send a Pay button. Substrings `⭐` and `XTR` in the buttons's text will be replaced with a Telegram Star icon. NOTE: This type of button must always be the first button in the first row and can only be used in invoice messages.
    #[must_use]
    pub fn pay_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.pay = val.map(Into::into);
        self
    }
}
