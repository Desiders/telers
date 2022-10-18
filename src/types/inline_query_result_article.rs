use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};

/// Represents a link to an article or web page.
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be *article*
    #[serde(rename = "type", default = "article")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>`_ attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. URL of the result
    pub url: Option<String>,
    /// *Optional*. Pass :code:`True` if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// *Optional*. Short description of the result
    pub description: Option<String>,
    /// *Optional*. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// *Optional*. Thumbnail width
    pub thumb_width: Option<i64>,
    /// *Optional*. Thumbnail height
    pub thumb_height: Option<i64>,
}

impl Default for InlineQueryResultArticle {
    fn default() -> Self {
        Self {
            result_type: article(),
            id: String::default(),
            title: String::default(),
            input_message_content: InputMessageContent::Text(Default::default()),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

fn article() -> String {
    "article".to_string()
}
