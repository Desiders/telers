use super::{Location, Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequery>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: Box<str>,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: Box<str>,
    /// Type of the chat from which the inline query was sent. Can be either 'sender' for a private chat with the inline query sender, 'private', 'group', 'supergroup', or 'channel'. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    pub chat_type: Option<Box<str>>,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
}

impl InlineQuery {
    /// Gets the sender user ID from the inline query
    #[must_use]
    pub const fn sender_user_id(&self) -> i64 {
        self.from.id
    }

    /// Gets the sender user ID from the inline query
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> i64 {
        self.sender_user_id()
    }
}

impl TryFrom<Update> for InlineQuery {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(inline_query) = update.inline_query {
            Ok(inline_query)
        } else {
            Err(ConvertUpdateToTypeError::new("InlineQuery"))
        }
    }
}
