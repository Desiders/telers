use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the [`content`](https://core.telegram.org/bots/api#inputmessagecontent) of a venue message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputvenuemessagecontent>
#[skip_serializing_none]
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
    /// Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [`supported types`](https://developers.google.com/places/web-service/supported_types).)
    pub google_place_type: Option<String>,
}

impl InputVenueMessageContent {
    #[must_use]
    pub fn new(
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
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
    pub fn address(self, val: impl Into<String>) -> Self {
        Self {
            address: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn foursquare_id(self, val: impl Into<String>) -> Self {
        Self {
            foursquare_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn foursquare_type(self, val: impl Into<String>) -> Self {
        Self {
            foursquare_type: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_id(self, val: impl Into<String>) -> Self {
        Self {
            google_place_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_type(self, val: impl Into<String>) -> Self {
        Self {
            google_place_type: Some(val.into()),
            ..self
        }
    }
}
