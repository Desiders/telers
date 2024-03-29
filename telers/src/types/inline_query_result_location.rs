use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the location.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultlocation>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}

impl InlineQueryResultLocation {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
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
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn longitude(self, val: f64) -> Self {
        Self {
            longitude: val,
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
    pub fn horizontal_accuracy(self, val: f64) -> Self {
        Self {
            horizontal_accuracy: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn live_period(self, val: i64) -> Self {
        Self {
            live_period: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn heading(self, val: i64) -> Self {
        Self {
            heading: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius(self, val: i64) -> Self {
        Self {
            proximity_alert_radius: Some(val),
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

impl InlineQueryResultLocation {
    #[must_use]
    pub fn horizontal_accuracy_option(self, val: Option<f64>) -> Self {
        Self {
            horizontal_accuracy: val,
            ..self
        }
    }

    #[must_use]
    pub fn live_period_option(self, val: Option<i64>) -> Self {
        Self {
            live_period: val,
            ..self
        }
    }

    #[must_use]
    pub fn heading_option(self, val: Option<i64>) -> Self {
        Self {
            heading: val,
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius_option(self, val: Option<i64>) -> Self {
        Self {
            proximity_alert_radius: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content_option(self, val: Option<impl Into<InputMessageContent>>) -> Self {
        Self {
            input_message_content: val.map(Into::into),
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
