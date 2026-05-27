use serde::{Deserialize, Serialize};
/// Represents a location to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedialocation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaLocation {
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
}
impl InputMediaLocation {
    /// Creates a new `InputMediaLocation`.
    ///
    /// # Arguments
    /// * `latitude` - Latitude of the location
    /// * `longitude` - Longitude of the location
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<f64>>(latitude: T0, longitude: T1) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            horizontal_accuracy: None,
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

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy<T: Into<f64>>(mut self, val: T) -> Self {
        self.horizontal_accuracy = Some(val.into());
        self
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy_option<T: Into<f64>>(mut self, val: Option<T>) -> Self {
        self.horizontal_accuracy = val.map(Into::into);
        self
    }
}
