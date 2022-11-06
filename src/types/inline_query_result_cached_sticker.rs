use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the sticker.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016 for static stickers and after 06 July, 2019 for `animated stickers <https://telegram.org/blog/animated-stickers>`. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedsticker>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be *sticker*
    #[serde(rename = "type", default = "sticker")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}

impl Default for InlineQueryResultCachedSticker {
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
    "sticker".to_string()
}
