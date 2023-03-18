use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InlineKeyboardMarkup, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send a game
/// # Documentation
/// <https://core.telegram.org/bots/api#sendgame>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via [Botfather](https://t.me/botfather).
    pub game_short_name: String,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one ‘Play game_title’ button will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendGame {
    #[must_use]
    pub fn new<T: Into<String>>(chat_id: i64, game_short_name: T) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            game_short_name: game_short_name.into(),
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = val;
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn game_short_name<T: Into<String>>(mut self, val: T) -> Self {
        self.game_short_name = val.into();
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl SendGame {
    #[must_use]
    pub fn message_thread_id_some(mut self, val: Option<i64>) -> Self {
        self.message_thread_id = val;
        self
    }

    #[must_use]
    pub fn disable_notification_some(mut self, val: Option<bool>) -> Self {
        self.disable_notification = val;
        self
    }

    #[must_use]
    pub fn protect_content_some(mut self, val: Option<bool>) -> Self {
        self.protect_content = val;
        self
    }

    #[must_use]
    pub fn reply_to_message_id_some(mut self, val: Option<i64>) -> Self {
        self.reply_to_message_id = val;
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply_some(mut self, val: Option<bool>) -> Self {
        self.allow_sending_without_reply = val;
        self
    }

    #[must_use]
    pub fn reply_markup_some<T: Into<InlineKeyboardMarkup>>(mut self, val: Option<T>) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}

impl TelegramMethod for SendGame {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendGame", self, None)
    }
}