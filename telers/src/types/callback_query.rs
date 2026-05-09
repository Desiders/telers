use serde::{Deserialize, Serialize};
/// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field `inline_message_id` will be present. Exactly one of the fields data or `game_short_name` will be present.
/// # Documentation
/// <https://core.telegram.org/bots/api#callbackquery>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: Box<crate::types::User>,
    /// Message sent by the bot with the callback button that originated the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<crate::types::MaybeInaccessibleMessage>>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: Box<str>,
    /// Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<str>>,
    /// Short name of a Game to be returned, serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<Box<str>>,
}
impl CallbackQuery {
    /// Creates a new `CallbackQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this query
    /// * `from` - Sender
    /// * `chat_instance` - Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::User>, T2: Into<Box<str>>>(
        id: T0,
        from: T1,
        chat_instance: T2,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            message: None,
            inline_message_id: None,
            chat_instance: chat_instance.into(),
            data: None,
            game_short_name: None,
        }
    }

    /// Unique identifier for this query
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Sender
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Message sent by the bot with the callback button that originated the query
    #[must_use]
    pub fn message<T: Into<crate::types::MaybeInaccessibleMessage>>(mut self, val: T) -> Self {
        self.message = Some(Box::new(val.into()));
        self
    }

    /// Message sent by the bot with the callback button that originated the query
    #[must_use]
    pub fn message_option<T: Into<crate::types::MaybeInaccessibleMessage>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }

    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    #[must_use]
    pub fn chat_instance<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_instance = val.into();
        self
    }

    /// Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = Some(val.into());
        self
    }

    /// Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[must_use]
    pub fn data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.data = val.map(Into::into);
        self
    }

    /// Short name of a Game to be returned, serves as the unique identifier for the game
    #[must_use]
    pub fn game_short_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.game_short_name = Some(val.into());
        self
    }

    /// Short name of a Game to be returned, serves as the unique identifier for the game
    #[must_use]
    pub fn game_short_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.game_short_name = val.map(Into::into);
        self
    }
}
