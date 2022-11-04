use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the venue.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultvenue>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be *venue*
    #[serde(rename = "type", default = "venue")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// *Optional*. Foursquare type of the venue, if known. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// *Optional*. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// *Optional*. Google Places type of the venue. (See `supported types <https://developers.google.com/places/web-service/supported_types>`.)
    pub google_place_type: Option<String>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// *Optional*. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// *Optional*. Thumbnail width
    pub thumb_width: Option<i64>,
    /// *Optional*. Thumbnail height
    pub thumb_height: Option<i64>,
}

impl Default for InlineQueryResultVenue {
    fn default() -> Self {
        Self {
            result_type: venue(),
            id: String::default(),
            latitude: 0.0,
            longitude: 0.0,
            title: String::default(),
            address: String::default(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

fn venue() -> String {
    "venue".to_string()
}
