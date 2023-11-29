use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an article or web page.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultArticle {
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
    /// Pass `true` if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Short description of the result
    pub description: Option<String>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
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
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
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
    pub fn thumbnail_url(self, val: impl Into<String>) -> Self {
        Self {
            thumbnail_url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_width(self, val: i64) -> Self {
        Self {
            thumbnail_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_height(self, val: i64) -> Self {
        Self {
            thumbnail_height: Some(val),
            ..self
        }
    }
}

impl InlineQueryResultArticle {
    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn hide_url_option(self, val: Option<bool>) -> Self {
        Self {
            hide_url: val,
            ..self
        }
    }

    #[must_use]
    pub fn description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            description: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            thumbnail_url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_width_option(self, val: Option<i64>) -> Self {
        Self {
            thumbnail_width: val,
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_height_option(self, val: Option<i64>) -> Self {
        Self {
            thumbnail_height: val,
            ..self
        }
    }
}
