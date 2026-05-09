use serde::{Deserialize, Serialize};
/// Represents a venue to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediavenue>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaVenue {
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// Name of the venue
    pub title: Box<str>,
    /// Address of the venue
    pub address: Box<str>,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<Box<str>>,
    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<Box<str>>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<Box<str>>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<Box<str>>,
}
impl InputMediaVenue {
    /// Creates a new `InputMediaVenue`.
    ///
    /// # Arguments
    /// * `latitude` - Latitude of the location
    /// * `longitude` - Longitude of the location
    /// * `title` - Name of the venue
    /// * `address` - Address of the venue
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<f64>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        latitude: T0,
        longitude: T1,
        title: T2,
        address: T3,
    ) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    /// Latitude of the location
    #[must_use]
    pub fn latitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.latitude = val.into();
        self
    }

    /// Longitude of the location
    #[must_use]
    pub fn longitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.longitude = val.into();
        self
    }

    /// Name of the venue
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Address of the venue
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.foursquare_id = Some(val.into());
        self
    }

    /// Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.foursquare_id = val.map(Into::into);
        self
    }

    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.foursquare_type = Some(val.into());
        self
    }

    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.foursquare_type = val.map(Into::into);
        self
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.google_place_id = Some(val.into());
        self
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.google_place_id = val.map(Into::into);
        self
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.google_place_type = Some(val.into());
        self
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.google_place_type = val.map(Into::into);
        self
    }
}
