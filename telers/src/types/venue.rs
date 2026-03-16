use serde::{Deserialize, Serialize};
/// This object represents a venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#venue>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: crate::types::Location,
    /// Name of the venue
    pub title: Box<str>,
    /// Address of the venue
    pub address: Box<str>,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<Box<str>>,
    /// Foursquare type of the venue. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<Box<str>>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<Box<str>>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<Box<str>>,
}
impl Venue {
    /// Creates a new `Venue`.
    ///
    /// # Arguments
    /// * `location` - Venue location. Can't be a live location
    /// * `title` - Name of the venue
    /// * `address` - Address of the venue
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        location: T0,
        title: T1,
        address: T2,
    ) -> Self {
        Self {
            location: location.into(),
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    /// Venue location. Can't be a live location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(self, val: T) -> Self {
        let mut this = self;
        this.location = val.into();
        this
    }

    /// Name of the venue
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Address of the venue
    #[must_use]
    pub fn address<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.address = val.into();
        this
    }

    /// Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.foursquare_id = Some(val.into());
        this
    }

    /// Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.foursquare_id = val.map(Into::into);
        this
    }

    /// Foursquare type of the venue. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.foursquare_type = Some(val.into());
        this
    }

    /// Foursquare type of the venue. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.foursquare_type = val.map(Into::into);
        this
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.google_place_id = Some(val.into());
        this
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.google_place_id = val.map(Into::into);
        this
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.google_place_type = Some(val.into());
        this
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.google_place_type = val.map(Into::into);
        this
    }
}
