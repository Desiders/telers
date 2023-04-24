use super::{InlineKeyboardMarkup, InputMessageContent};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the location.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultlocation>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    /// Type of the result, must be *location*
    #[serde(rename = "type", default = "location")]
    pub result_type: String,
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
    pub thumb_url: Option<String>,
    /// Thumbnail width
    pub thumb_width: Option<i64>,
    /// Thumbnail height
    pub thumb_height: Option<i64>,
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

impl Default for InlineQueryResultLocation {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: location(),
            id: String::default(),
            latitude: 0.0,
            longitude: 0.0,
            title: String::default(),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

fn location() -> String {
    InlineQueryResultType::Location.into()
}
