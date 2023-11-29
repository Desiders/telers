use super::{Message, Update, UpdateKind, User};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object represents an incoming callback query from a callback button in an [`inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field `message` will be present. If the button was attached to a message sent via the bot (in [`inline mode`](https://core.telegram.org/bots/api#inline-mode)), the field `inline_message_id` will be present. Exactly one of the fields *data* or `game_short_name` will be present.
/// **NOTE:** After the user presses a callback button, Telegram clients will display a progress bar until you call [`AnswerCallbackQuery`](crate::methods::AnswerCallbackQuery). It is, therefore, necessary to react by calling [`AnswerCallbackQuery`](crate::methods::AnswerCallbackQuery) even if no notification to the user is needed (e.g., without specifying any of the optional parameters).
/// # Documentation
/// <https://core.telegram.org/bots/api#callbackquery>
/// # Warnings
/// This structure has so big size, so it's recommended to use it inside [`std::sync::Arc`], [`Box`] and other smart pointers
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: User,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [`games`](https://core.telegram.org/bots/api#games).
    pub chat_instance: Box<str>,
    /// Message with the callback button that originated the query. Note that message content and message date will not be available if the message is too old
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<Box<str>>,
    /// Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    pub data: Option<Box<str>>,
    /// Short name of a [`Game`](https://core.telegram.org/bots/api#games) to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<Box<str>>,
}

impl CallbackQuery {
    #[must_use]
    pub const fn chat_id(&self) -> Option<i64> {
        if let Some(message) = &self.message {
            Some(message.chat().id())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn message_id(&self) -> Option<i64> {
        if let Some(message) = &self.message {
            Some(message.id())
        } else {
            None
        }
    }

    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        if let Some(message) = &self.message {
            message.text()
        } else {
            None
        }
    }

    #[must_use]
    pub fn message_caption(&self) -> Option<&str> {
        if let Some(message) = &self.message {
            message.caption()
        } else {
            None
        }
    }

    #[must_use]
    pub fn message_text_or_caption(&self) -> Option<&str> {
        if let Some(message) = &self.message {
            message.text_or_caption()
        } else {
            None
        }
    }
}

impl TryFrom<Update> for CallbackQuery {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::CallbackQuery(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "CallbackQuery")),
        }
    }
}
