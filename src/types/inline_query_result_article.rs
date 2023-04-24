use super::{InlineKeyboardMarkup, InputMessageContent, InputTextMessageContent};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an article or web page.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>
#[skip_serializing_none]
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
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    pub url: Option<String>,
    /// Pass `True` if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Short description of the result
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultArticle {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        input_message_content: impl Into<InputMessageContent>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content: input_message_content.into(),
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
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(self, val: impl Into<InputMessageContent>) -> Self {
        Self {
            input_message_content: val.into(),
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
    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn hide_url(self, val: bool) -> Self {
        Self {
            hide_url: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumb_url(self, val: impl Into<String>) -> Self {
        Self {
            thumb_url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumb_width(self, val: i64) -> Self {
        Self {
            thumb_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumb_height(self, val: i64) -> Self {
        Self {
            thumb_height: Some(val),
            ..self
        }
    }
}

impl Default for InlineQueryResultArticle {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: article(),
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

fn article() -> String {
    InlineQueryResultType::Article.into()
}
