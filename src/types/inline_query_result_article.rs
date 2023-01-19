use super::{InlineKeyboardMarkup, InputMessageContent, InputTextMessageContent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an article or web page.
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be *article*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. URL of the result
    pub url: Option<String>,
    /// *Optional*. Pass `True` if you don't want the URL to be shown in the message
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

impl InlineQueryResultArticle {
    #[must_use]
    pub fn new<T: Into<String>>(
        id: T,
        title: T,
        input_message_content: InputMessageContent,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = val;
        self
    }

    #[must_use]
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    #[must_use]
    pub fn url<T: Into<String>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    #[must_use]
    pub fn hide_url(mut self, val: bool) -> Self {
        self.hide_url = Some(val);
        self
    }

    #[must_use]
    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    #[must_use]
    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = Some(val.into());
        self
    }

    #[must_use]
    pub fn thumb_width(mut self, val: i64) -> Self {
        self.thumb_width = Some(val);
        self
    }

    #[must_use]
    pub fn thumb_height(mut self, val: i64) -> Self {
        self.thumb_height = Some(val);
        self
    }
}

impl Default for InlineQueryResultArticle {
    fn default() -> Self {
        Self {
            result_type: "article".to_string(),
            id: String::default(),
            title: String::default(),
            input_message_content: InputMessageContent::Text(InputTextMessageContent::default()),
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
