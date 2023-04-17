use super::{CallbackGame, LoginUrl, WebAppInfo};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents one button of an inline keyboard. You **must** use exactly one of the optional fields.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardbutton>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their ID without using a username, if this is allowed by their privacy settings.
    pub url: Option<String>,
    /// Data to be sent in a [`callback query`](https://core.telegram.org/bots/api#callbackquery) to the bot when button is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// Description of the [`Web App`](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [`AnswerWebAppQuery`](crate::methods::AnswerWebAppQuery). Available only in private chats between a user and the bot.
    pub web_app: Option<WebAppInfo>,
    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [`Telegram Login Widget`](https://core.telegram.org/widgets/login).
    pub login_url: Option<LoginUrl>,
    /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted.
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.
    pub switch_inline_query_current_chat: Option<String>,
    /// Description of the game that will be launched when the user presses the button.
    pub callback_game: Option<CallbackGame>,
    /// Specify `True`, to send a [`Pay button`](https://core.telegram.org/bots/api#payments).
    pub pay: Option<bool>,
}

impl InlineKeyboardButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
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
    pub fn url(mut self, val: impl Into<String>) -> Self {
        self.url = Some(val.into());
        self
    }

    #[must_use]
    pub fn callback_data(mut self, val: impl Into<String>) -> Self {
        self.callback_data = Some(val.into());
        self
    }

    #[must_use]
    pub fn web_app(mut self, val: WebAppInfo) -> Self {
        self.web_app = Some(val);
        self
    }

    #[must_use]
    pub fn login_url(mut self, val: LoginUrl) -> Self {
        self.login_url = Some(val);
        self
    }

    #[must_use]
    pub fn switch_inline_query(mut self, val: impl Into<String>) -> Self {
        self.switch_inline_query = Some(val.into());
        self
    }

    #[must_use]
    pub fn switch_inline_query_current_chat(mut self, val: impl Into<String>) -> Self {
        self.switch_inline_query_current_chat = Some(val.into());
        self
    }

    #[must_use]
    pub fn callback_game(mut self, val: CallbackGame) -> Self {
        self.callback_game = Some(val);
        self
    }

    #[must_use]
    pub fn pay(mut self, val: bool) -> Self {
        self.pay = Some(val);
        self
    }
}
