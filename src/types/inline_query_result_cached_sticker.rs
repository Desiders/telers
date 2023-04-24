use super::{InlineKeyboardMarkup, InputMessageContent};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the sticker.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016 for static stickers and after 06 July, 2019 for [`animated stickers`](https://telegram.org/blog/animated-stickers). Older clients will ignore them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedsticker>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be *sticker*
    #[serde(rename = "type", default = "sticker")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedSticker {
    #[must_use]
    pub fn new(id: impl Into<String>, sticker_file_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker_file_id(self, val: impl Into<String>) -> Self {
        Self {
            sticker_file_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(self, val: impl Into<InputMessageContent>) -> Self {
        Self {
            input_message_content: Some(val.into()),
            ..self
        }
    }
}

impl Default for InlineQueryResultCachedSticker {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: sticker(),
            id: String::default(),
            sticker_file_id: String::default(),
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn sticker() -> String {
    InlineQueryResultType::Sticker.into()
}
