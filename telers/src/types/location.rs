use serde::{Deserialize, Serialize};
/// This object represents a point on the map.
/// # Documentation
/// <https://core.telegram.org/bots/api#location>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Latitude as defined by the sender
    pub latitude: f64,
    /// Longitude as defined by the sender
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}
impl Location {
    /// Creates a new `Location`.
    ///
    /// # Arguments
    /// * `latitude` - Latitude as defined by the sender
    /// * `longitude` - Longitude as defined by the sender
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

    /// Latitude as defined by the sender
    #[must_use]
    pub fn latitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.latitude = val.into();
        this
    }

    /// Longitude as defined by the sender
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

    /// Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[must_use]
    pub fn live_period<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.live_period = Some(val.into());
        this
    }

    /// Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[must_use]
    pub fn live_period_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.live_period = val.map(Into::into);
        this
    }

    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[must_use]
    pub fn heading<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.heading = Some(val.into());
        this
    }

    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[must_use]
    pub fn heading_option<T: Into<u16>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.heading = val.map(Into::into);
        this
    }

    /// The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[must_use]
    pub fn proximity_alert_radius<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.proximity_alert_radius = Some(val.into());
        this
    }

    /// The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[must_use]
    pub fn proximity_alert_radius_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.proximity_alert_radius = val.map(Into::into);
        this
    }
}
