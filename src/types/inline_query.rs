use super::{Location, Update, User};

use serde::Deserialize;

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequery>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// *Optional*. Type of the chat from which the inline query was sent. Can be either 'sender' for a private chat with the inline query sender, 'private', 'group', 'supergroup', or 'channel'. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    pub chat_type: Option<String>,
    /// *Optional*. Sender location, only for bots that request user location
    pub location: Option<Location>,
}

impl From<Update> for InlineQuery {
    fn from(update: Update) -> Self {
        update.inline_query.expect("Update is not an `InlineQuery`")
    }
}
