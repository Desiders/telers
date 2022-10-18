use super::MessageEntity;

use serde::{Deserialize, Serialize};

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>`_ of a text message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputtextmessagecontent>_
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// *Optional*. Mode for parsing entities in the message text. See `formatting options <https://core.telegram.org/bots/api#formatting-options>`_ for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in message text, which can be specified instead of *parse_mode*
    pub entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}
