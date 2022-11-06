use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the location.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultlocation>
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

impl Default for InlineQueryResultLocation {
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
    "location".to_string()
}
