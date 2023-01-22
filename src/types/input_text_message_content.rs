use super::MessageEntity;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a text message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputtextmessagecontent>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// *Optional*. Mode for parsing entities in the message text. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in message text, which can be specified instead of *parse_mode*
    pub entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

impl InputTextMessageContent {
    #[must_use]
    pub fn new<T: Into<String>>(message_text: T) -> Self {
        Self {
            message_text: message_text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }

    #[must_use]
    pub fn message_text<T: Into<String>>(mut self, val: T) -> Self {
        self.message_text = val.into();
        self
    }

    #[must_use]
    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.entities = Some(val);
        self
    }

    #[must_use]
    pub fn disable_web_page_preview(mut self, val: bool) -> Self {
        self.disable_web_page_preview = Some(val);
        self
    }
}
