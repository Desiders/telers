use super::Location;

use serde::Deserialize;

/// This object represents a venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#venue>
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [`supported types`](https://developers.google.com/places/web-service/supported_types).)
    pub google_place_type: Option<String>,
}
