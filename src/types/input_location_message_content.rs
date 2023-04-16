use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the [`content`](https://core.telegram.org/bots/api#inputmessagecontent) of a location message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputlocationmessagecontent>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// *Optional*. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
}

impl InputLocationMessageContent {
    #[must_use]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    #[must_use]
    pub fn horizontal_accuracy(self, val: f64) -> Self {
        Self {
            horizontal_accuracy: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn live_period(self, val: i64) -> Self {
        Self {
            live_period: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn heading(self, val: i64) -> Self {
        Self {
            heading: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius(self, val: i64) -> Self {
        Self {
            proximity_alert_radius: Some(val),
            ..self
        }
    }
}
