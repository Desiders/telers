use super::{InlineKeyboardMarkup, InputMessageContent};

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
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// *Optional*. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// *Optional*. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// *Optional*. Thumbnail width
    pub thumb_width: Option<i64>,
    /// *Optional*. Thumbnail height
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultLocation {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, latitude: f64, longitude: f64, title: T) -> Self {
        Self {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn horizontal_accuracy(mut self, val: f64) -> Self {
        self.horizontal_accuracy = Some(val);
        self
    }

    #[must_use]
    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
        self
    }

    #[must_use]
    pub fn heading(mut self, val: i64) -> Self {
        self.heading = Some(val);
        self
    }

    #[must_use]
    pub fn proximity_alert_radius(mut self, val: i64) -> Self {
        self.proximity_alert_radius = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
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

impl Default for InlineQueryResultLocation {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: "location".to_string(),
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
