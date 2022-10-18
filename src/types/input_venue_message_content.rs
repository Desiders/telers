use serde::{Deserialize, Serialize};

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>`_ of a venue message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputvenuemessagecontent>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    /// *Optional*. Google Places type of the venue. (See `supported types <https://developers.google.com/places/web-service/supported_types>`_.)
    pub google_place_type: Option<String>,
}

impl Default for InputVenueMessageContent {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            title: String::default(),
            address: String::default(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}
