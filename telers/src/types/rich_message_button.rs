use serde::{Deserialize, Serialize};
/// This object represents a button in a [`crate::types::RichMessage`]. Exactly one of the fields other than text and style must be used to specify the type of the button.
/// # Documentation
/// <https://core.telegram.org/bots/api#richmessagebutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichMessageButton {
    /// Text of the button. May contain only plain text, [`crate::types::RichTextCustomEmoji`] and [`crate::types::RichTextDateTime`] entities.
    pub text: Box<crate::types::RichText>,
    /// Style of the button. Must be one of `danger`, `success`, `primary`, or `link` (the button is shown as a regular link without borders). Apps may use theme-specific colors for the button background and text based on the style. The style `link` is allowed only for callback buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<str>>,
    /// HTTP or `tg`:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// Data to be sent in a callback query to the bot when the button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<Box<str>>,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [`crate::methods::AnswerWebAppQuery`]. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::types::WebAppInfo>,
    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget. Not supported for ephemeral messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<crate::types::LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<Box<str>>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<Box<str>>,
    /// If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<crate::types::SwitchInlineQueryChosenChat>,
    /// A button that copies the specified text to the clipboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<crate::types::CopyTextButton>,
    /// If set, then the button is disabled and does nothing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<crate::types::DisabledButton>,
}
impl RichMessageButton {
    /// Creates a new `RichMessageButton`.
    ///
    /// # Arguments
    /// * `text` - Text of the button. May contain only plain text, [`crate::types::RichTextCustomEmoji`] and [`crate::types::RichTextDateTime`] entities.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
            style: None,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            disabled: None,
        }
    }

    /// Text of the button. May contain only plain text, [`crate::types::RichTextCustomEmoji`] and [`crate::types::RichTextDateTime`] entities.
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// Style of the button. Must be one of `danger`, `success`, `primary`, or `link` (the button is shown as a regular link without borders). Apps may use theme-specific colors for the button background and text based on the style. The style `link` is allowed only for callback buttons.
    #[must_use]
    pub fn style<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.style = Some(val.into());
        self
    }

    /// Style of the button. Must be one of `danger`, `success`, `primary`, or `link` (the button is shown as a regular link without borders). Apps may use theme-specific colors for the button background and text based on the style. The style `link` is allowed only for callback buttons.
    #[must_use]
    pub fn style_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.style = val.map(Into::into);
        self
    }

    /// HTTP or `tg`:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    /// HTTP or `tg`:// URL to be opened when the button is pressed. Links ``tg://user?id=<user_id>`` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
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

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [`crate::methods::AnswerWebAppQuery`]. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[must_use]
    pub fn web_app<T: Into<crate::types::WebAppInfo>>(mut self, val: T) -> Self {
        self.web_app = Some(val.into());
        self
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [`crate::methods::AnswerWebAppQuery`]. Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[must_use]
    pub fn web_app_option<T: Into<crate::types::WebAppInfo>>(mut self, val: Option<T>) -> Self {
        self.web_app = val.map(Into::into);
        self
    }

    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget. Not supported for ephemeral messages.
    #[must_use]
    pub fn login_url<T: Into<crate::types::LoginUrl>>(mut self, val: T) -> Self {
        self.login_url = Some(val.into());
        self
    }

    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget. Not supported for ephemeral messages.
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

    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[must_use]
    pub fn switch_inline_query_current_chat<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.switch_inline_query_current_chat = Some(val.into());
        self
    }

    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
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

    /// A button that copies the specified text to the clipboard
    #[must_use]
    pub fn copy_text<T: Into<crate::types::CopyTextButton>>(mut self, val: T) -> Self {
        self.copy_text = Some(val.into());
        self
    }

    /// A button that copies the specified text to the clipboard
    #[must_use]
    pub fn copy_text_option<T: Into<crate::types::CopyTextButton>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.copy_text = val.map(Into::into);
        self
    }

    /// If set, then the button is disabled and does nothing
    #[must_use]
    pub fn disabled<T: Into<crate::types::DisabledButton>>(mut self, val: T) -> Self {
        self.disabled = Some(val.into());
        self
    }

    /// If set, then the button is disabled and does nothing
    #[must_use]
    pub fn disabled_option<T: Into<crate::types::DisabledButton>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.disabled = val.map(Into::into);
        self
    }
}
