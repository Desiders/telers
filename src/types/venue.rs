use super::Location;

use serde::{Deserialize, Serialize};

/// This object represents a venue.
/// <https://core.telegram.org/bots/api#venue>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// *Optional*. Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// *Optional*. Foursquare type of the venue. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// *Optional*. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// *Optional*. Google Places type of the venue. (See `supported types <https://developers.google.com/places/web-service/supported_types>`_.)
    pub google_place_type: Option<String>,
}

impl Default for Venue {
    fn default() -> Self {
        Self {
            location: Location::default(),
            title: String::default(),
            address: String::default(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}
