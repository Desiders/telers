use super::{CallbackGame, LoginUrl, WebAppInfo};

use serde::{Deserialize, Serialize};

/// This object represents one button of an inline keyboard. You **must** use exactly one of the optional fields.
/// <https://core.telegram.org/bots/api#inlinekeyboardbutton>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// *Optional*. HTTP or tg:// URL to be opened when the button is pressed. Links :code:`tg://user?id=<user_id>` can be used to mention a user by their ID without using a username, if this is allowed by their privacy settings.
    pub url: Option<String>,
    /// *Optional*. Data to be sent in a `callback query <https://core.telegram.org/bots/api#callbackquery>`_ to the bot when button is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// *Optional*. Description of the `Web App <https://core.telegram.org/bots/webapps>`_ that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method :class:`aiogram_rs.methods.answer_web_app_query.AnswerWebAppQuery`. Available only in private chats between a user and the bot.
    pub web_app: Option<WebAppInfo>,
    /// *Optional*. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the `Telegram Login Widget <https://core.telegram.org/widgets/login>`_.
    pub login_url: Option<LoginUrl>,
    /// *Optional*. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted.
    pub switch_inline_query: Option<String>,
    /// *Optional*. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.
    pub switch_inline_query_current_chat: Option<String>,
    /// *Optional*. Description of the game that will be launched when the user presses the button.
    pub callback_game: Option<CallbackGame>,
    /// *Optional*. Specify :code:`True`, to send a `Pay button <https://core.telegram.org/bots/api#payments>`_.
    pub pay: Option<bool>,
}
