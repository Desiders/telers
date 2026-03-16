use serde::{Deserialize, Serialize};
/// Represents the content of a location message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputlocationmessagecontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}
impl InputLocationMessageContent {
    /// Creates a new `InputLocationMessageContent`.
    ///
    /// # Arguments
    /// * `latitude` - Latitude of the location in degrees
    /// * `longitude` - Longitude of the location in degrees
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<f64>>(latitude: T0, longitude: T1) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    /// Latitude of the location in degrees
    #[must_use]
    pub fn latitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.latitude = val.into();
        this
    }

    /// Longitude of the location in degrees
    #[must_use]
    pub fn longitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.longitude = val.into();
        this
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.horizontal_accuracy = Some(val.into());
        this
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy_option<T: Into<f64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.horizontal_accuracy = val.map(Into::into);
        this
    }

    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[must_use]
    pub fn live_period<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.live_period = Some(val.into());
        this
    }

    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[must_use]
    pub fn live_period_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.live_period = val.map(Into::into);
        this
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.heading = Some(val.into());
        this
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading_option<T: Into<u16>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.heading = val.map(Into::into);
        this
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.proximity_alert_radius = Some(val.into());
        this
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.proximity_alert_radius = val.map(Into::into);
        this
    }
}
