use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a venue message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputvenuemessagecontent>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// *Optional*. Foursquare type of the venue, if known. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// *Optional*. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// *Optional*. Google Places type of the venue. (See `supported types <https://developers.google.com/places/web-service/supported_types>`.)
    pub google_place_type: Option<String>,
}
